## Remaining issues.

- `xs::types::SimpleRestrictionType` has a sub-group that does not collapse.

## Facet support

### ALL

assertions

#### Lexical space

pattern: regex

#### Value space:

enumeration: options

### Ordered

float, int,

MaxInclusive/MaxExclusive
MinInclusive/MinExclusive

### Length

#### Strings

length: characters
minLength: -::-
maxLength: -::-
whiteSpace: preserve (nothing), replace (replaces LF, tabs and CR with space), collapse

#### AnyURI

length: characters
minLength: -::-
maxLength: -::-

#### hexBinary && base64Binary

length: byte count
minLength: -::-
maxLength: -::-

#### lists

length: item count
minLength: -::-
maxLength: -::-
whiteSpace = "collapse"

#### Decimal

totalDigits
fractionDigits
