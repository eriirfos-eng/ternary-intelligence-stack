# Mapping from IR to binary opcodes (v1.2.5 spec)
OPCODE_MAP = {
    "TV_ADD": b"\x01",
    "TV_MUL": b"\x02",
    "TV_TRELU": b"\x03",
    "TSYNC": b"\x04",
    "TCONSENSUS": b"\x05",
}

def generate_tbc(ir: str):
    bytecode = b""
    lines = ir.split("\n")
    for line in lines:
        if line in OPCODE_MAP:
            bytecode += OPCODE_MAP[line]
    return bytecode
