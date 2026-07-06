(set-logic QF_LIA)
(set-option :produce-models true)

; Extracted from:
; - tlmcmddb/src/tlm.rs
; - tlmcmddb-csv/src/tlm/body.rs
;
; tlm::FieldGroup documents that when a group contains multiple fields, its
; onboard variable type must be an unsigned integer.  The CSV parser currently
; builds the data model from rows but does not validate the group-level bit
; layout.  This finite abstraction uses at most two fields, enough to witness
; non-unsigned multi-field groups, out-of-width fields, and overlapping fields.
(declare-const fieldCount Int)
(declare-const variableBitWidth Int)
(declare-const variableIsUnsignedInteger Bool)
(declare-const field1Start Int)
(declare-const field1Length Int)
(declare-const field2Start Int)
(declare-const field2Length Int)

(assert (or (= fieldCount 1) (= fieldCount 2)))
(assert (or (= variableBitWidth 8)
            (= variableBitWidth 16)
            (= variableBitWidth 32)
            (= variableBitWidth 64)))

; Rust usize deserialization already prevents negative positions and lengths.
(assert (>= field1Start 0))
(assert (>= field1Length 0))
(assert (>= field2Start 0))
(assert (>= field2Length 0))

; Only uint8_t/uint16_t/uint32_t are unsigned integer variable types in the data
; model.  float/double and signed integers are represented by
; variableIsUnsignedInteger = false in this abstraction.
(assert (=> variableIsUnsignedInteger
            (or (= variableBitWidth 8)
                (= variableBitWidth 16)
                (= variableBitWidth 32))))

(define-fun RangeValid ((start Int) (len Int)) Bool
  (and (> len 0)
       (<= (+ start len) variableBitWidth)))

(define-fun RangesOverlap () Bool
  (and (< field1Start (+ field2Start field2Length))
       (< field2Start (+ field1Start field1Length))))

; Given a row that deserializes and satisfies conversion-info checks, the current
; parser has no extra group-level range validation.
(define-fun CurrentParserAccepts () Bool true)

; Candidate domain contract for safe field-group extraction/codegen.
(define-fun CandidateAccepts () Bool
  (and (RangeValid field1Start field1Length)
       (or (= fieldCount 1)
           (and variableIsUnsignedInteger
                (RangeValid field2Start field2Length)
                (not RangesOverlap)))))

; 1. Current parser can accept a multi-field group whose variable type is not an
; unsigned integer, even though the data-model comment says it should be.
(push)
(assert (= fieldCount 2))
(assert (= variableBitWidth 16))
(assert (not variableIsUnsignedInteger))
(assert (= field1Start 0))
(assert (= field1Length 8))
(assert (= field2Start 8))
(assert (= field2Length 8))
(assert CurrentParserAccepts)
(assert (not CandidateAccepts))
(check-sat)
(get-model)
(pop)

; 2. Current parser can accept a field that extends beyond the variable width.
(push)
(assert (= fieldCount 1))
(assert (= variableBitWidth 8))
(assert (not variableIsUnsignedInteger))
(assert (= field1Start 7))
(assert (= field1Length 2))
(assert CurrentParserAccepts)
(assert (not CandidateAccepts))
(check-sat)
(get-model)
(pop)

; 3. Current parser can accept overlapping fields in the same group.
(push)
(assert (= fieldCount 2))
(assert (= variableBitWidth 8))
(assert variableIsUnsignedInteger)
(assert (= field1Start 0))
(assert (= field1Length 4))
(assert (= field2Start 3))
(assert (= field2Length 4))
(assert RangesOverlap)
(assert CurrentParserAccepts)
(assert (not CandidateAccepts))
(check-sat)
(get-model)
(pop)

; 4. Sanity: the candidate contract accepts a single scalar field that exactly
; fills a signed or floating-point width.
(push)
(assert (= fieldCount 1))
(assert (= variableBitWidth 32))
(assert (not variableIsUnsignedInteger))
(assert (= field1Start 0))
(assert (= field1Length 32))
(assert CandidateAccepts)
(check-sat)
(get-model)
(pop)

; 5. Candidate multi-field groups never overlap.
(push)
(assert (= fieldCount 2))
(assert CandidateAccepts)
(assert RangesOverlap)
(check-sat)
(pop)

; 6. Candidate accepted field1 never exceeds the group variable width.
(push)
(assert CandidateAccepts)
(assert (> (+ field1Start field1Length) variableBitWidth))
(check-sat)
(pop)

; 7. Candidate multi-field groups require an unsigned integer variable type.
(push)
(assert (= fieldCount 2))
(assert (not variableIsUnsignedInteger))
(assert CandidateAccepts)
(check-sat)
(pop)
