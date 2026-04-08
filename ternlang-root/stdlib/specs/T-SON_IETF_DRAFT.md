Internet Engineering Task Force (IETF)                        S. Chairman
Internet-Draft                                                RFI-IRFOS
Intended status: Standards Track                         April 08, 2026
Expires: October 08, 2026

                   T-SON: Ternary JSON Data Interchange
                     draft-rfi-irfos-tson-01

Abstract

   This document specifies the Ternary JavaScript Object Notation
   (T-SON) data interchange format. T-SON is a lightweight, text-based,
   language-independent data interchange format derived from JSON,
   extended to support balanced ternary logic natively.

   It introduces a foundational primitive type, the 'trit', capable of
   representing three distinct mathematical states: 'affirm', 'reject',
   and 'tend' (equilibrium/ambiguity). This format is designed for the
   transmission of uncertainty-aware network payloads without the
   degradation inherent in binary coercion.

1. Introduction

   As computational paradigms shift toward post-binary architectures,
   particularly within the Ternary Intelligence Stack (TIS), traditional
   JSON (RFC 8259) forces triadic intelligence models to coerce
   uncertainty into binary `true`, `false`, or `null`.

   T-SON solves this impedance mismatch by embedding the 'trit' as a
   first-class scalar type alongside strings, numbers, booleans, and null.

2. MIME Type

   The MIME media type for T-SON text is application/tson.
   The default encoding is UTF-8.

3. The Trit Primitive

   T-SON introduces three literal names for the trit primitive. These
   names MUST be lowercase.

      trit = "affirm" / "reject" / "tend"

   Semantics:
   *  `affirm` (1): Positive state, truth, or logical high.
   *  `reject` (-1): Negative state, falsehood, or logical low.
   *  `tend` (0): Equilibrium state, ambiguity, unknown, or middle.

   Note: `tend` is NOT semantically equivalent to `null`. `null` represents
   the absence of a value, whereas `tend` is an explicit, computable
   mathematical state denoting maximum uncertainty.

4. Data Structures

   T-SON retains all structural components of JSON:
   * Object: A collection of name/value pairs.
   * Array: An ordered list of values.

5. Syntax Example

   The following is a valid T-SON object demonstrating standard JSON
   types alongside the new trit primitives.

   {
     "model_id": "TIS-MoE-13",
     "active": true,
     "confidence_threshold": 0.85,
     "inference_gates": {
       "vision_sensor": affirm,
       "lidar_sensor": tend,
       "audio_sensor": reject
     },
     "fallback_tensor": [tend, affirm, tend, reject]
   }

6. Serialization and Parsers

   Parsers encountering a T-SON payload MUST map `affirm`, `reject`,
   and `tend` to the underlying hardware's trit representation (e.g.,
   int8 values of 1, -1, 0, respectively, if executing on legacy
   binary hardware, or direct voltage levels on TernCore-Silicon).

7. Security Considerations

   T-SON inherits the security considerations of JSON. However, parsers
   MUST ensure that `tend` values do not inadvertently trigger null-pointer
   dereferences in weakly-typed binary environments.

8. Author's Address

   Simeon, Chairman
   Research Focus Institute - Interdisciplinary Research Facility for Open Sciences
   Elisabethinergasse 25, 8020 Graz, Austria
   URI: https://bazaarak.com
