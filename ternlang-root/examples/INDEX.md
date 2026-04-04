# Ternlang Examples

A growing collection of `.tern` programs demonstrating real-world decision logic in balanced ternary.

Every example follows the same pattern: three states (`-1` / `0` / `+1`) map to something concrete. The magic is always in the middle value — the state that binary systems are forced to throw away.

---

## Quick Reference

| State | Trit | Also called | Meaning |
|-------|------|-------------|---------|
| Reject | `-1` | `conflict()` | Clear negative signal. Do not proceed. |
| Hold | `0` | `hold()` | Not enough data. Wait or ask for more. |
| Affirm | `+1` | `truth()` | Clear positive signal. Proceed. |

---

## Examples

### Fundamentals

| # | File | What it shows |
|---|------|---------------|
| 01 | [hello_trit.tern](01_hello_trit.tern) | All three trit values, `invert()`, `consensus()` — start here |
| 02 | [decision_gate.tern](02_decision_gate.tern) | Safety as a hard gate: safety conflict blocks everything else |

### Real-World Decisions

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 03 | [rocket_launch.tern](03_rocket_launch.tern) | Aerospace | Go / No-Go / Hold; range safety as absolute veto |
| 04 | [sensor_fusion.tern](04_sensor_fusion.tern) | Autonomous vehicles | Four-sensor fusion; any obstacle signal dominates |
| 05 | [medical_triage.tern](05_medical_triage.tern) | Healthcare | ER triage; consciousness as hard gate |
| 06 | [git_merge.tern](06_git_merge.tern) | DevOps | CI as hard gate; auto-merge / review / block |
| 07 | [spam_filter.tern](07_spam_filter.tern) | Email | Quarantine ≠ spam folder; hold is an active routing label |
| 08 | [evidence_collector.tern](08_evidence_collector.tern) | AI agents | Low data density detection; formal "I need more" signal |

### Computer Science & Systems

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 09 | [risc_fetch_decode.tern](09_risc_fetch_decode.tern) | CPU / Systems | Fetch-decode-execute pipeline; stall = hold; inspired by Brandon Smith's 9-trit RISC simulator |
| 13 | [owlet_bridge.tern](13_owlet_bridge.tern) | Programming languages | Ternary S-expression eval loop; suspended eval = hold; inspired by the Owlet interpreter |
| 14 | [circuit_breaker.tern](14_circuit_breaker.tern) | Microservices | HALF-OPEN state is natively trit = 0; no special-casing needed |

### Human Decisions & Civic Systems

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 10 | [confidence_escalator.tern](10_confidence_escalator.tern) | AI agents | Self-assessment before answering; escalate when uncertain |
| 11 | [form_validator.tern](11_form_validator.tern) | UX / Web | Empty ≠ invalid; ternary UX avoids hostile "required field" errors |
| 12 | [vote_aggregator.tern](12_vote_aggregator.tern) | Civic / Governance | Abstain is signal, not silence; quorum detection |
| 15 | [loan_underwriter.tern](15_loan_underwriter.tern) | Finance | Approve / refer to human / decline; automated humility |
| 16 | [content_moderation.tern](16_content_moderation.tern) | Trust & Safety | Allow / review / remove; human in the loop for the hold zone |
| 18 | [treaty_negotiation.tern](18_treaty_negotiation.tern) | Diplomacy | Veto ≠ reserve; failed ratification vs. procedural hold |
| 20 | [hiring_pipeline.tern](20_hiring_pipeline.tern) | HR / Recruiting | Hold bucket is the most valuable stage; references as soft gate |

### Infrastructure & Engineering

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 17 | [job_scheduler.tern](17_job_scheduler.tern) | Systems | Defer ≠ cancel; resource pressure produces hold, not cancellation |
| 19 | [cache_invalidation.tern](19_cache_invalidation.tern) | Web / CDN | Stale-while-revalidate is natively trit = 0; named properly at last |

---

## Patterns Demonstrated

### Hard Gate
A signal so important that its negative value overrides everything else:
```
match critical_signal {
    -1 => { return conflict(); }   // veto — no further evaluation needed
     0 => { ... }
     1 => { ... }
}
```
Used in: rocket launch (range safety), medical triage (consciousness), spam filter (blocklist), loan underwriting (DTI ratio).

### Density Check
When fewer than N of M signals are decisive, request more data instead of guessing:
```
// if not enough decisive signals → return hold()
// "I don't know yet — here's what I need"
```
Used in: `08_evidence_collector.tern`, `10_confidence_escalator.tern`, `12_vote_aggregator.tern`.

### Cascading Consensus
Chain `consensus(a, b)` calls to aggregate multiple signals:
```
let ab:  trit = consensus(signal_a, signal_b);
let abc: trit = consensus(ab, signal_c);
```
Used in: nearly every example. The workhorse of ternary aggregation.

### Hold as Routing Label
`hold()` is not "undecided" — it is a first-class output value that tells the caller what to do next:
- Spam filter: quarantine folder
- Circuit breaker: probe mode
- Content moderation: human review queue
- Loan underwriting: human underwriter queue
- Form validator: show "required" hint, not error

---

## Contributing

New examples follow the naming convention `NN_snake_case_name.tern`.

Every example should:
1. Have a header comment explaining the real-world scenario
2. Demonstrate what binary systems get wrong (and why ternary fixes it)
3. Have a concrete scenario at the end that can be traced through manually
4. Return a meaningful trit via a top-level `match` block

---

## Attribution

- `09_risc_fetch_decode.tern` — conceptually informed by Brandon Smith's Python 9-trit RISC simulator
- `13_owlet_bridge.tern` — conceptually informed by the Owlet S-expression ternary interpreter (Node.js)
- Balanced ternary mathematical foundations: Knuth (1997), *The Art of Computer Programming*
- Physical ternary precedent: Setun computer, Moscow State University, 1958
- BitNet b1.58 ternary neural network weights: Ma et al. (2024)

### Newly Added Examples (21-100)

| # | File | Domain / What it shows |
|---|------|------------------------|
| 100 | [100_event_weather.tern](100_event_weather.tern) | Event cancellation weather gate |
| 21 | [21_nuclear_reactor.tern](21_nuclear_reactor.tern) | Nuclear reactor SCRAM / HOLD / NORMAL decision |
| 22 | [22_bridge_structural_health.tern](22_bridge_structural_health.tern) | Bridge structural health monitoring |
| 23 | [23_elevator_safety_interlock.tern](23_elevator_safety_interlock.tern) | Elevator safety interlock |
| 24 | [24_chemical_plant_pressure.tern](24_chemical_plant_pressure.tern) | Chemical plant pressure relief valve |
| 25 | [25_dam_water_level.tern](25_dam_water_level.tern) | Dam water level management |
| 26 | [26_power_grid_frequency.tern](26_power_grid_frequency.tern) | Power grid frequency stability |
| 27 | [27_wind_turbine_fatigue.tern](27_wind_turbine_fatigue.tern) | Wind turbine blade fatigue monitoring |
| 28 | [28_oil_pipeline_leak.tern](28_oil_pipeline_leak.tern) | Oil pipeline leak detection |
| 29 | [29_aircraft_deicing.tern](29_aircraft_deicing.tern) | Aircraft deicing decision |
| 30 | [30_runway_incursion.tern](30_runway_incursion.tern) | Runway incursion detection |
| 31 | [31_drug_interaction.tern](31_drug_interaction.tern) | Drug interaction checker |
| 32 | [32_icu_ventilator.tern](32_icu_ventilator.tern) | ICU ventilator weaning readiness |
| 33 | [33_sepsis_warning.tern](33_sepsis_warning.tern) | Sepsis early warning |
| 34 | [34_radiology_flag.tern](34_radiology_flag.tern) | Radiology report flag |
| 35 | [35_clinical_trial.tern](35_clinical_trial.tern) | Clinical trial eligibility screening |
| 36 | [36_organ_transplant.tern](36_organ_transplant.tern) | Organ transplant compatibility |
| 37 | [37_surgical_checklist.tern](37_surgical_checklist.tern) | Surgical go/no-go checklist |
| 38 | [38_antibiotic_resistance.tern](38_antibiotic_resistance.tern) | Antibiotic resistance risk |
| 39 | [39_mental_health_triage.tern](39_mental_health_triage.tern) | Mental health crisis triage |
| 40 | [40_apgar_ternary.tern](40_apgar_ternary.tern) | Neonatal APGAR-inspired ternary score |
| 41 | [41_insurance_claim.tern](41_insurance_claim.tern) | Automated Claims Processing |
| 42 | [42_algorithmic_trading.tern](42_algorithmic_trading.tern) | Algorithmic trading signal |
| 42 | [42_trading_signal.tern](42_trading_signal.tern) | Volatility-Aware Market Decision |
| 43 | [43_aml_transaction.tern](43_aml_transaction.tern) | Anti-Money Laundering (AML) Filter |
| 44 | [44_options_expiry.tern](44_options_expiry.tern) | Options Trading Settlement |
| 45 | [45_portfolio_rebalance.tern](45_portfolio_rebalance.tern) | Wealth Management Drift Control |
| 46 | [46_atc_conflict.tern](46_atc_conflict.tern) | Air traffic control conflict alert |
| 46 | [46_startup_due_diligence.tern](46_startup_due_diligence.tern) | Venture Capital Filter |
| 47 | [47_railway_signal.tern](47_railway_signal.tern) | Railway signal block occupancy |
| 47 | [47_fraud_detection.tern](47_fraud_detection.tern) | E-commerce Payment Integrity |
| 48 | [48_central_bank_rate.tern](48_central_bank_rate.tern) | Monetary Policy Decision |
| 48 | [48_autonomous_lane_change.tern](48_autonomous_lane_change.tern) | Autonomous vehicle lane change |
| 49 | [49_port_customs.tern](49_port_customs.tern) | Port container customs clearance |
| 49 | [49_crypto_withdrawal.tern](49_crypto_withdrawal.tern) | Digital Asset Custody Gate |
| 50 | [50_drone_flight.tern](50_drone_flight.tern) | Drone flight authorization |
| 50 | [50_invoice_authorization.tern](50_invoice_authorization.tern) | Accounts Payable Workflow |
| 51 | [51_bail_decision.tern](51_bail_decision.tern) | Pre-trial Justice Algorithm |
| 52 | [52_patent_prior_art.tern](52_patent_prior_art.tern) | Patent prior art check |
| 52 | [52_parole_review.tern](52_parole_review.tern) | Corrections Rehabilitation Assessment |
| 53 | [53_building_code.tern](53_building_code.tern) | Building code compliance inspection |
| 53 | [53_patent_prior_art.tern](53_patent_prior_art.tern) | Intellectual Property Examination |
| 54 | [54_contract_clause_risk.tern](54_contract_clause_risk.tern) | Legal Document Analysis |
| 54 | [54_environmental_permit.tern](54_environmental_permit.tern) | Environmental permit approval |
| 55 | [55_immigration_visa.tern](55_immigration_visa.tern) | Border Control & Talent Mobility |
| 55 | [55_whistleblower_triage.tern](55_whistleblower_triage.tern) | Whistleblower complaint triage |
| 56 | [56_wildfire_risk.tern](56_wildfire_risk.tern) | Wildfire risk assessment |
| 57 | [57_air_quality.tern](57_air_quality.tern) | Air quality index action |
| 58 | [58_drought_irrigation.tern](58_drought_irrigation.tern) | Drought irrigation trigger |
| 59 | [59_crop_disease.tern](59_crop_disease.tern) | Crop disease detection |
| 60 | [60_soil_contamination.tern](60_soil_contamination.tern) | Soil contamination classification |
| 61 | [61_mfa_authentication.tern](61_mfa_authentication.tern) | Multi-factor authentication |
| 62 | [62_biometric_liveness.tern](62_biometric_liveness.tern) | Biometric liveness detection |
| 63 | [63_firewall_rule.tern](63_firewall_rule.tern) | Firewall rule hit classification |
| 64 | [64_ransomware_behavior.tern](64_ransomware_behavior.tern) | Ransomware behavior detection |
| 65 | [65_insider_threat.tern](65_insider_threat.tern) | Insider threat behavioral flag |
| 66 | [66_adaptive_test.tern](66_adaptive_test.tern) | Adaptive test difficulty gate |
| 67 | [67_student_at_risk.tern](67_student_at_risk.tern) | Student at-risk early warning |
| 68 | [68_scholarship_scoring.tern](68_scholarship_scoring.tern) | Scholarship eligibility scoring |
| 69 | [69_academic_integrity.tern](69_academic_integrity.tern) | Academic integrity flag |
| 70 | [70_peer_review.tern](70_peer_review.tern) | Paper peer-review recommendation |
| 71 | [71_solar_dispatch.tern](71_solar_dispatch.tern) | Solar panel dispatch decision |
| 72 | [72_battery_storage.tern](72_battery_storage.tern) | Battery storage charge/discharge gate |
| 73 | [73_smart_meter_anomaly.tern](73_smart_meter_anomaly.tern) | Smart meter anomaly detection |
| 74 | [74_ev_charging.tern](74_ev_charging.tern) | EV charging session authorization |
| 75 | [75_gas_pressure.tern](75_gas_pressure.tern) | Gas pressure regulator valve |
| 76 | [76_emergency_shelter.tern](76_emergency_shelter.tern) | Emergency shelter allocation |
| 77 | [77_food_bank.tern](77_food_bank.tern) | Food bank eligibility |
| 78 | [78_refugee_status.tern](78_refugee_status.tern) | Refugee status determination |
| 79 | [79_elder_care.tern](79_elder_care.tern) | Elder care assessment |
| 80 | [80_noise_complaint.tern](80_noise_complaint.tern) | Noise complaint escalation |
| 81 | [81_housing_benefit.tern](81_housing_benefit.tern) | Housing benefit eligibility |
| 82 | [82_api_rate_limit.tern](82_api_rate_limit.tern) | API rate limit enforcement |
| 83 | [83_database_query.tern](83_database_query.tern) | Database query classification |
| 84 | [84_deployment_gate.tern](84_deployment_gate.tern) | Deployment readiness gate |
| 85 | [85_ab_test_gate.tern](85_ab_test_gate.tern) | A/B test significance gate |
| 86 | [86_bug_triage.tern](86_bug_triage.tern) | Bug severity triage |
| 87 | [87_code_review.tern](87_code_review.tern) | Code review approval gate |
| 88 | [88_dependency_vulnerability.tern](88_dependency_vulnerability.tern) | Dependency vulnerability check |
| 89 | [89_container_health.tern](89_container_health.tern) | Container health probe |
| 90 | [90_feature_flag.tern](90_feature_flag.tern) | Feature flag rollout gate |
| 91 | [91_dns_resolution.tern](91_dns_resolution.tern) | DNS resolution confidence |
| 92 | [92_referee_challenge.tern](92_referee_challenge.tern) | Referee challenge review |
| 93 | [93_athlete_injury.tern](93_athlete_injury.tern) | Athlete injury risk before match |
| 94 | [94_doping_test.tern](94_doping_test.tern) | Doping test result gate |
| 95 | [95_film_rating.tern](95_film_rating.tern) | Film rating board classification |
| 96 | [96_music_rights.tern](96_music_rights.tern) | Music rights clearance |
| 97 | [97_live_streaming.tern](97_live_streaming.tern) | Live streaming quality adaptation |
| 98 | [98_esports_anti_cheat.tern](98_esports_anti_cheat.tern) | Esports anti-cheat classification |
| 99 | [99_horse_racing.tern](99_horse_racing.tern) | Horse racing track condition flag |
