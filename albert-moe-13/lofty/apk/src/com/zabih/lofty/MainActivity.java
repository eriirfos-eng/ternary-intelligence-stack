package com.zabih.lofty;

import android.Manifest;
import android.annotation.SuppressLint;
import android.app.Activity;
import android.bluetooth.*;
import android.bluetooth.le.*;
import android.content.pm.PackageManager;
import android.graphics.Color;
import android.os.*;
import android.view.View;
import android.view.animation.*;
import android.widget.*;
import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;
import javax.crypto.Cipher;
import javax.crypto.spec.SecretKeySpec;

@SuppressLint({"MissingPermission"})
public class MainActivity extends Activity {

    // ── BLE identifiers ──────────────────────────────────────────────────────
    private static final String SVC        = "0000fff0-0000-1000-8000-00805f9b34fb";
    private static final String CHR_NOTIFY = "0000fff1-0000-1000-8000-00805f9b34fb";
    private static final String CHR_WRITE  = "0000fff2-0000-1000-8000-00805f9b34fb";
    private static final String TARGET     = "jida_bike";

    // ── Jida protocol keys (extracted from APK v2.1.5) ──────────────────────
    private static final byte[] DEVICE_KEY = {
        0x0b, (byte)0xf2, (byte)0xda, 0x3e, (byte)0xef, (byte)0xf4, (byte)0xef, (byte)0x82,
        0x34, 0x03, (byte)0xb9, (byte)0x97, (byte)0xfa, 0x2b, 0x5c, 0x52
    };
    private static final byte[] PHONE_ENC_KEY = {17,18,19,20,17,18,19,20,17,18,19,20,17,18,19,20};

    // Zabih's registered phone (stored locally, no network needed)
    private static final String PHONE = "67764823881";

    private static final int PERM_REQ = 1;

    // ── State ─────────────────────────────────────────────────────────────────
    private BluetoothLeScanner    scanner;
    private BluetoothGatt         gatt;
    private BluetoothGattCharacteristic writeChar;
    private final AtomicBoolean   scanning = new AtomicBoolean(false);
    private final Handler         ui       = new Handler(Looper.getMainLooper());
    private Handler               worker;

    // ── Views ─────────────────────────────────────────────────────────────────
    private TextView     tvStatus, tvLog, tvRingIcon;
    private Button       btnUnlock, btnLock;
    private ProgressBar  pbScan;

    // ── Activity lifecycle ────────────────────────────────────────────────────
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        HandlerThread wt = new HandlerThread("lofty-ble");
        wt.start();
        worker = new Handler(wt.getLooper());

        tvStatus  = findViewById(R.id.tv_status);
        tvLog     = findViewById(R.id.tv_log);
        tvRingIcon = findViewById(R.id.tv_ring_icon);
        pbScan    = findViewById(R.id.pb_scan);
        btnUnlock = findViewById(R.id.btn_unlock);
        btnLock   = findViewById(R.id.btn_lock);

        btnUnlock.setOnClickListener(new View.OnClickListener() { public void onClick(View v) { startFlow(); } });
        btnLock.setOnClickListener(new View.OnClickListener() { public void onClick(View v) { sendLock(); } });

        BluetoothManager bm = (BluetoothManager) getSystemService(BLUETOOTH_SERVICE);
        scanner = bm.getAdapter().getBluetoothLeScanner();

        checkPermissionsAndStart();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        stopScan();
        if (gatt != null) { gatt.close(); gatt = null; }
    }

    // ── Permission handling ───────────────────────────────────────────────────
    private void checkPermissionsAndStart() {
        List<String> needed = new ArrayList<>();
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (checkSelfPermission(Manifest.permission.BLUETOOTH_SCAN)    != PackageManager.PERMISSION_GRANTED) needed.add(Manifest.permission.BLUETOOTH_SCAN);
            if (checkSelfPermission(Manifest.permission.BLUETOOTH_CONNECT) != PackageManager.PERMISSION_GRANTED) needed.add(Manifest.permission.BLUETOOTH_CONNECT);
        } else {
            if (checkSelfPermission(Manifest.permission.BLUETOOTH)         != PackageManager.PERMISSION_GRANTED) needed.add(Manifest.permission.BLUETOOTH);
            if (checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) != PackageManager.PERMISSION_GRANTED) needed.add(Manifest.permission.ACCESS_FINE_LOCATION);
        }
        if (needed.isEmpty()) {
            startFlow();
        } else {
            requestPermissions(needed.toArray(new String[0]), PERM_REQ);
        }
    }

    @Override
    public void onRequestPermissionsResult(int code, String[] perms, int[] results) {
        if (code == PERM_REQ) {
            boolean ok = true;
            for (int r : results) if (r != PackageManager.PERMISSION_GRANTED) { ok = false; break; }
            if (ok) startFlow(); else setStatus("need BT permissions", "#F87171");
        }
    }

    // ── Main flow: scan → connect → auth → unlock ─────────────────────────────
    private void setScanSpinner(final boolean on) {
        ui.post(new Runnable() { public void run() {
            pbScan.setVisibility(on ? View.VISIBLE : View.GONE);
            if (on) {
                AlphaAnimation pulse = new AlphaAnimation(1f, 0.3f);
                pulse.setDuration(700);
                pulse.setRepeatMode(Animation.REVERSE);
                pulse.setRepeatCount(Animation.INFINITE);
                tvRingIcon.startAnimation(pulse);
            } else {
                tvRingIcon.clearAnimation();
            }
        } });
    }

    private void startFlow() {
        if (scanning.get() || gatt != null) return;
        setStatus("scanning…", "#F59E0B");
        appendLog("scanning for " + TARGET + "…");
        btnUnlock.setEnabled(false);
        setScanSpinner(true);
        startScan();
    }

    private void startScan() {
        scanning.set(true);
        ScanFilter filter = new ScanFilter.Builder()
            .setDeviceName(TARGET)
            .build();
        ScanSettings settings = new ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
            .build();
        scanner.startScan(Collections.singletonList(filter), settings, scanCallback);
    }

    private void stopScan() {
        if (scanning.getAndSet(false)) {
            try { scanner.stopScan(scanCallback); } catch (Exception ignored) {}
        }
    }

    private final LoftyScanCallback  scanCallback = new LoftyScanCallback();
    private final LoftyGattCallback  gattCallback = new LoftyGattCallback();

    private class LoftyScanCallback extends ScanCallback {
        @Override
        public void onScanResult(int callbackType, ScanResult result) {
            stopScan();
            setScanSpinner(false);
            BluetoothDevice dev = result.getDevice();
            appendLog("found: " + dev.getName() + " " + dev.getAddress());
            setStatus("connecting…", "#F59E0B");
            ui.post(new Runnable() {
                public void run() {
                    gatt = dev.connectGatt(MainActivity.this, false, gattCallback, BluetoothDevice.TRANSPORT_LE);
                }
            });
        }
        @Override
        public void onScanFailed(int errorCode) {
            setScanSpinner(false);
            appendLog("scan failed: " + errorCode);
            setStatus("scan error " + errorCode, "#F87171");
            ui.post(new Runnable() { public void run() { btnUnlock.setEnabled(true); } });
        }
    }

    private class LoftyGattCallback extends BluetoothGattCallback {
        @Override
        public void onConnectionStateChange(BluetoothGatt g, int status, int newState) {
            if (newState == BluetoothProfile.STATE_CONNECTED) {
                appendLog("GATT connected");
                setStatus("discovering…", "#F59E0B");
                g.discoverServices();
            } else {
                setScanSpinner(false);
                appendLog("disconnected (status=" + status + ")");
                setStatus("disconnected", "#64748B");
                gatt = null; writeChar = null;
                ui.post(new Runnable() { public void run() { btnUnlock.setEnabled(true); btnLock.setEnabled(false); } });
            }
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt g, int status) {
            BluetoothGattService svc = g.getService(UUID.fromString(SVC));
            if (svc == null) { appendLog("service not found"); return; }
            writeChar = svc.getCharacteristic(UUID.fromString(CHR_WRITE));
            BluetoothGattCharacteristic notifyChr = svc.getCharacteristic(UUID.fromString(CHR_NOTIFY));
            g.setCharacteristicNotification(notifyChr, true);
            BluetoothGattDescriptor desc = notifyChr.getDescriptors().isEmpty()
                ? null : notifyChr.getDescriptors().get(0);
            if (desc != null) {
                desc.setValue(BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE);
                g.writeDescriptor(desc);
            } else {
                appendLog("no descriptor — proceeding");
                doAuthFlow();
            }
        }

        @Override
        public void onDescriptorWrite(BluetoothGatt g, BluetoothGattDescriptor d, int status) {
            appendLog("notify enabled");
            doAuthFlow();
        }

        @Override
        public void onCharacteristicWrite(BluetoothGatt g, BluetoothGattCharacteristic c, int status) {
            appendLog("write " + (status == BluetoothGatt.GATT_SUCCESS ? "ok" : "fail " + status));
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt g, BluetoothGattCharacteristic c) {
            byte[] v = c.getValue();
            appendLog("rx " + hex(v));
            // Response format: [88][01][len][00][06][subtype][echoed_cmd][result][...]
            if (v.length > 7) {
                int cmd  = v[6] & 0xFF;
                int code = v[7] & 0xFF;
                if (cmd == 0x16) {
                    if (code == 0x00) {
                        appendLog("auth OK");
                        setStatus("authenticated", "#22D3EE");
                        worker.postDelayed(new SendUnlockRunnable(), 200);
                    } else {
                        appendLog("auth FAIL code=" + code);
                        setStatus("auth failed", "#F87171");
                    }
                } else if (cmd == 0x1B) {
                    appendLog(code == 0x00 ? "unlock ACK" : "lock ACK");
                }
            }
        }
    }

    // ── Auth + unlock sequence ─────────────────────────────────────────────────
    private void doAuthFlow() {
        setStatus("authenticating…", "#22D3EE");
        worker.post(new Runnable() { public void run() {
            try {
                writeCmd(new byte[]{0x00, 0x01, 0x27, 0x00});
                Thread.sleep(300);
                byte[] auth = buildAuthCmd(PHONE);
                appendLog("tx auth (" + auth.length + " bytes)");
                writeCmd(buildPayload(new byte[]{0x00, 0x01, 0x16, 0x27}, auth));
            } catch (Exception e) {
                appendLog("auth error: " + e.getMessage());
            }
        } });
    }

    private class SendUnlockRunnable implements Runnable { public void run() { sendUnlock(); } }

    private void sendUnlock() {
        try {
            writeCmd(new byte[]{0x00, 0x01, 0x1b, 0x01, 0x01});
            setStatus("unlocked", "#4ADE80");
            ui.post(new Runnable() { public void run() { btnLock.setEnabled(true); btnUnlock.setEnabled(true); } });
            appendLog("unlock sent");
        } catch (Exception e) {
            appendLog("unlock error: " + e.getMessage());
        }
    }

    private void sendLock() {
        worker.post(new Runnable() { public void run() {
            try {
                writeCmd(new byte[]{0x00, 0x01, 0x1b, 0x01, 0x00});
                Thread.sleep(300);
                writeCmd(new byte[]{0x00, 0x01, 0x12, 0x00});
                Thread.sleep(200);
                if (gatt != null) { gatt.disconnect(); gatt.close(); gatt = null; }
                setStatus("locked", "#64748B");
                ui.post(new Runnable() { public void run() { btnLock.setEnabled(false); btnUnlock.setEnabled(true); } });
                appendLog("locked + disconnected");
            } catch (Exception e) {
                appendLog("lock error: " + e.getMessage());
            }
        } });
    }

    // ── Protocol helpers ──────────────────────────────────────────────────────
    // getCmd: frame = [0x88, 0x01, len+1, ...payload, sum_mod256_checksum]
    private byte[] getCmd(byte[] payload) {
        byte[] frame = new byte[payload.length + 4];
        frame[0] = (byte) 0x88;
        frame[1] = 0x01;
        frame[2] = (byte) ((payload.length + 1) & 0xFF);
        System.arraycopy(payload, 0, frame, 3, payload.length);
        int sum = 0;
        for (int i = 0; i < frame.length - 1; i++) sum += frame[i] & 0xFF;
        frame[frame.length - 1] = (byte) (sum & 0xFF);
        return frame;
    }

    private void writeCmd(byte[] payload) {
        byte[] frame = getCmd(payload);
        appendLog("tx " + hex(frame));
        if (writeChar != null && gatt != null) {
            writeChar.setValue(frame);
            writeChar.setWriteType(BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE);
            gatt.writeCharacteristic(writeChar);
        }
    }

    // concat header + body into a single payload for writeCmd
    private byte[] buildPayload(byte[] header, byte[] body) {
        byte[] out = new byte[header.length + body.length];
        System.arraycopy(header, 0, out, 0, header.length);
        System.arraycopy(body, 0, out, header.length, body.length);
        return out;
    }

    // Build the 39-byte auth body: DEVICE_KEY(16) + encPhone(16) + timestamp(7)
    private byte[] buildAuthCmd(String phone) throws Exception {
        // 1. phone → 8-byte big-endian integer → 16 hex chars, doubled to 16 bytes
        long phoneNum = Long.parseLong(phone.replaceAll("\\D", ""));
        String phoneHex = String.format("%016x", phoneNum); // 16 hex chars = 8 bytes
        byte[] block = new byte[16];
        for (int i = 0; i < 8; i++) {
            byte b = (byte) Integer.parseInt(phoneHex.substring(i * 2, i * 2 + 2), 16);
            block[i] = b; block[i + 8] = b; // doubled
        }

        // 2. AES-128-ECB encrypt block with PHONE_ENC_KEY
        Cipher cipher = Cipher.getInstance("AES/ECB/NoPadding");
        cipher.init(Cipher.ENCRYPT_MODE, new SecretKeySpec(PHONE_ENC_KEY, "AES"));
        byte[] enc = cipher.doFinal(block);

        // 3. Timestamp: [year_hi, year_lo, month, day, hour, min, sec]
        Calendar cal = Calendar.getInstance();
        int year = cal.get(Calendar.YEAR);
        byte[] ts = new byte[]{
            (byte)(year / 100), (byte)(year % 100),
            (byte)(cal.get(Calendar.MONTH) + 1),
            (byte) cal.get(Calendar.DAY_OF_MONTH),
            (byte) cal.get(Calendar.HOUR_OF_DAY),
            (byte) cal.get(Calendar.MINUTE),
            (byte) cal.get(Calendar.SECOND)
        };

        // 4. body = DEVICE_KEY + enc + ts (39 bytes total = 0x27)
        byte[] body = new byte[39];
        System.arraycopy(DEVICE_KEY, 0, body, 0,  16);
        System.arraycopy(enc,        0, body, 16, 16);
        System.arraycopy(ts,         0, body, 32,  7);

        // header 00 01 16 27 is prepended by doAuthFlow, return body only
        return body;
    }

    // ── UI helpers ─────────────────────────────────────────────────────────────
    private void setStatus(final String msg, final String colorHex) {
        ui.post(new Runnable() { public void run() {
            tvStatus.setText(msg);
            tvStatus.setTextColor(Color.parseColor(colorHex));
        } });
    }

    private final StringBuilder logBuf = new StringBuilder();
    private void appendLog(final String msg) {
        ui.post(new Runnable() { public void run() {
            logBuf.append(msg).append('\n');
            String[] lines = logBuf.toString().split("\n");
            if (lines.length > 6) {
                logBuf.setLength(0);
                for (int i = lines.length - 6; i < lines.length; i++)
                    logBuf.append(lines[i]).append('\n');
            }
            tvLog.setText(logBuf.toString());
        } });
    }

    private static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte v : b) sb.append(String.format("%02x ", v));
        return sb.toString().trim();
    }

}
