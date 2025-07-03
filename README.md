## Remaining issues.

- `xs::types::SimpleRestrictionType` has a sub-group that does not collapse.
- In XHTML, lots of elements have complex types that are the exact same. This generates lots of useless code. By unifing it into a single common type somehow, we could save thousands of lines of code - atleast around 5-6k just from types like `kbd`.

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
