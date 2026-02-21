# output-formatting Specification

## Purpose
TBD - created by archiving change refactor-to-rust. Update Purpose after archive.
## Requirements
### Requirement: ASCII Formatting
The system SHALL provide ASCII-only output for maximum compatibility.

#### Scenario: ASCII multiplication symbol
- **WHEN** formatting `"2 * 3"` in ASCII mode
- **THEN** output uses `"*"` for multiplication

#### Scenario: ASCII division symbol
- **WHEN** formatting `"6 / 2"` in ASCII mode
- **THEN** output uses `"/"` for division

#### Scenario: ASCII square root
- **WHEN** formatting `"sqrt(16)"` in ASCII mode
- **THEN** output uses `"sqrt()"` notation

#### Scenario: ASCII square symbol
- **WHEN** formatting exponent in ASCII mode
- **THEN** output uses `"^"` notation: `"x^2"`

#### Scenario: ASCII complex number
- **WHEN** formatting complex number in ASCII mode
- **THEN** format is `"a + bi"` or `"a - bi"`

#### Scenario: ASCII fraction
- **WHEN** formatting fraction in ASCII mode
- **THEN** format is `"a/b"` or integer if denominator is 1

#### Scenario: ASCII pi constant
- **WHEN** formatting pi in ASCII mode
- **THEN** output is numeric value or `"pi"` text

### Requirement: Unicode Formatting
The system SHALL provide Unicode output with mathematical symbols for better readability.

#### Scenario: Unicode multiplication symbol
- **WHEN** formatting `"2 * 3"` in Unicode mode
- **THEN** output uses `"×"` for multiplication

#### Scenario: Unicode division symbol
- **WHEN** formatting `"6 / 2"` in Unicode mode
- **THEN** output uses `"÷"` for division

#### Scenario: Unicode pi
- **WHEN** formatting pi in Unicode mode
- **THEN** output uses `"π"` symbol

#### Scenario: Unicode square root
- **WHEN** formatting `"sqrt(16)"` in Unicode mode
- **THEN** output uses `"√"` symbol: `"√16"`

#### Scenario: Unicode superscript 2
- **WHEN** formatting `"x^2"` in Unicode mode
- **THEN** output uses `"x²"`

#### Scenario: Unicode superscript 3
- **WHEN** formatting `"x^3"` in Unicode mode
- **THEN** output uses `"x³"`

#### Scenario: Unicode subscript 0
- **WHEN** formatting variable with index 0 in Unicode mode
- **THEN** output uses `"x₀"`

#### Scenario: Unicode subscript 1
- **WHEN** formatting variable with index 1 in Unicode mode
- **THEN** output uses `"x₁"`

#### Scenario: Unicode subscript 2
- **WHEN** formatting variable with index 2 in Unicode mode
- **THEN** output uses `"x₂"`

#### Scenario: Unicode subscript 3
- **WHEN** formatting variable with index 3 in Unicode mode
- **THEN** output uses `"x₃"`

#### Scenario: Unicode subscript 4
- **WHEN** formatting variable with index 4 in Unicode mode
- **THEN** output uses `"x₄"`

#### Scenario: Unicode subscript 5
- **WHEN** formatting variable with index 5 in Unicode mode
- **THEN** output uses `"x₅"`

#### Scenario: Unicode complex number
- **WHEN** formatting complex number in Unicode mode
- **THEN** format uses Unicode symbols for operations

#### Scenario: Unicode fraction
- **WHEN** formatting fraction in Unicode mode
- **THEN** format is `"a/b"` with Unicode operations

### Requirement: LaTeX Formatting
The system SHALL generate LaTeX source code for typeset output.

#### Scenario: LaTeX delimiters
- **WHEN** generating LaTeX output
- **THEN** result is wrapped in `"\\[ ... \\]"`

#### Scenario: LaTeX multiplication
- **WHEN** formatting multiplication in LaTeX
- **THEN** implicit multiplication or `"\\cdot"` is used

#### Scenario: LaTeX square root
- **WHEN** formatting square root in LaTeX
- **THEN** output uses `"\\sqrt{x}"` notation

#### Scenario: LaTeX exponent
- **WHEN** formatting exponent in LaTeX
- **THEN** output uses `"x^{2}"` notation

#### Scenario: LaTeX subscript
- **WHEN** formatting subscript in LaTeX
- **THEN** output uses `"x_{1}"` notation

#### Scenario: LaTeX fraction
- **WHEN** formatting fraction in LaTeX
- **THEN** output uses `"\\frac{a}{b}"` notation

#### Scenario: LaTeX complex number
- **WHEN** formatting complex number in LaTeX
- **THEN** output uses LaTeX math notation

#### Scenario: LaTeX quadratic formula
- **WHEN** formatting quadratic solution in LaTeX
- **THEN** output uses proper LaTeX math syntax

### Requirement: Prompt Formatting
The system SHALL format the interactive prompt according to output mode.

#### Scenario: ASCII prompt
- **WHEN** in ASCII mode
- **THEN** prompt is `">>> "`

#### Scenario: Unicode prompt
- **WHEN** in Unicode mode
- **THEN** prompt uses Unicode symbol like `"➜ "` or `"∫ "`

#### Scenario: LaTeX prompt
- **WHEN** in LaTeX mode
- **THEN** prompt uses appropriate Unicode or LaTeX symbol

#### Scenario: No prompt in non-interactive mode
- **WHEN** evaluating single expression from command line
- **THEN** no prompt is displayed

### Requirement: Pretty Output Configuration
The system SHALL provide global configuration for output formatting.

#### Scenario: Set ASCII mode
- **WHEN** user specifies `--ascii` flag
- **THEN** all output uses ASCII formatting

#### Scenario: Set Unicode mode
- **WHEN** user specifies `--unicode` flag
- **THEN** all output uses Unicode formatting

#### Scenario: Set auto-detect mode
- **WHEN** user specifies `--pretty` flag
- **THEN** system detects terminal capabilities and selects best format

#### Scenario: Set LaTeX mode
- **WHEN** user specifies `--latex` flag
- **THEN** LaTeX code is generated

#### Scenario: Get current mode
- **WHEN** formatting output
- **THEN** current pretty level is used

### Requirement: Terminal Capability Detection
The system SHALL detect terminal capabilities for automatic format selection.

#### Scenario: Detect Kitty protocol support
- **WHEN** terminal supports Kitty graphics protocol
- **THEN** LaTeX rendering is considered available

#### Scenario: Detect standard terminal
- **WHEN** terminal does not support graphics
- **THEN** Unicode is selected if available

#### Scenario: Detect limited terminal
- **WHEN** terminal has limited Unicode support
- **THEN** ASCII is selected

#### Scenario: Detect pdflatex availability
- **WHEN** checking for LaTeX tools
- **THEN** pdflatex command availability is checked

#### Scenario: Auto-select Unicode
- **WHEN** `--pretty` is specified and Unicode is available
- **THEN** Unicode mode is selected

#### Scenario: Auto-select LaTeX
- **WHEN** `--pretty` is specified and LaTeX + Kitty are available
- **THEN** LaTeX mode is selected

#### Scenario: Fallback to ASCII
- **WHEN** Unicode or LaTeX are unavailable
- **THEN** ASCII mode is used

### Requirement: LaTeX Rendering
The system SHALL render LaTeX output to images in supporting terminals.

#### Scenario: Generate LaTeX source
- **WHEN** formatting in LaTeX mode
- **THEN** valid LaTeX source code is generated

#### Scenario: Compile to PDF
- **WHEN** pdflatex is available
- **THEN** LaTeX source is compiled to PDF

#### Scenario: Convert to PNG
- **WHEN** LaTeX PDF is generated
- **THEN** PDF is converted to PNG image

#### Scenario: Base64 encode image
- **WHEN** PNG image is ready
- **THEN** image is Base64 encoded

#### Scenario: Display via Kitty protocol
- **WHEN** terminal supports Kitty protocol
- **THEN** image is displayed inline

#### Scenario: Skip rendering if unavailable
- **WHEN** pdflatex or image conversion tools are not available
- **THEN** LaTeX source code is displayed instead

### Requirement: Equation Solution Formatting
The system SHALL format equation solutions consistently across output modes.

#### Scenario: Single solution
- **WHEN** displaying one equation solution
- **THEN** format is `"x = value"`

#### Scenario: Multiple solutions
- **WHEN** displaying multiple solutions
- **THEN** format is `"x1 = value1, x2 = value2, ..."`

#### Scenario: Linear system solutions
- **WHEN** displaying linear system solutions
- **THEN** format is `"x = value1, y = value2, z = value3"`

#### Scenario: Subscripts in Unicode
- **WHEN** using Unicode mode for multiple solutions
- **THEN** variables use subscripts: `"x₁ = 2, x₂ = 3"`

#### Scenario: Subscripts in LaTeX
- **WHEN** using LaTeX mode for multiple solutions
- **THEN** variables use subscript notation: `"x_{1} = 2, x_{2} = 3"`

#### Scenario: Exact fractions in solutions
- **WHEN** solution is rational
- **THEN** exact fraction is displayed: `"x = 3/2"`

#### Scenario: Complex solutions
- **WHEN** solution is complex
- **THEN** format is `"a + bi"` or `"a - bi"`

### Requirement: Error Message Formatting
The system SHALL format error messages consistently.

#### Scenario: Error prefix
- **WHEN** an error occurs
- **THEN** message starts with `"Error:"`

#### Scenario: Parse error
- **WHEN** expression cannot be parsed
- **THEN** error describes the syntax issue

#### Scenario: Evaluation error
- **WHEN** expression cannot be evaluated
- **THEN** error describes the evaluation problem

#### Scenario: Equation solving error
- **WHEN** equation cannot be solved
- **THEN** error describes why solving failed

#### Scenario: Division by zero error
- **WHEN** division by zero is attempted
- **THEN** error message indicates division by zero

### Requirement: Output Mode Priority
The system SHALL respect explicit output mode flags over auto-detection.

#### Scenario: Explicit ASCII overrides auto-detect
- **WHEN** user specifies `--ascii` even though Unicode is available
- **THEN** ASCII mode is used

#### Scenario: Explicit Unicode overrides auto-detect
- **WHEN** user specifies `--unicode` even though LaTeX is available
- **THEN** Unicode mode is used

#### Scenario: Explicit LaTeX overrides auto-detect
- **WHEN** user specifies `--latex` even if terminal doesn't support graphics
- **THEN** LaTeX source code is generated

#### Scenario: Default to ASCII
- **WHEN** no flags are specified
- **THEN** ASCII mode is used (default behavior)

