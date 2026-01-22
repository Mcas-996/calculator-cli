# Spec: CLI Interface

## ADDED Requirements

### Requirement: Argument Parsing
The system SHALL parse command-line arguments for configuration and expression evaluation.

#### Scenario: Evaluate expression from argument
- **WHEN** providing expression `"3 + 5 * 2"` as argument
- **THEN** expression is evaluated and result is printed

#### Scenario: Interactive mode without argument
- **WHEN** no expression is provided
- **THEN** interactive REPL mode is entered

#### Scenario: Multiple arguments
- **WHEN** providing multiple expressions
- **THEN** only the last expression is evaluated

#### Scenario: Expression with spaces
- **WHEN** providing `"3 + 5 * 2"` as argument
- **THEN** entire argument is treated as single expression

#### Scenario: Expression with quotes
- **WHEN** providing `'3 + 5 * 2'` with single quotes
- **THEN** quotes are stripped and expression is evaluated

### Requirement: Help Flag
The system SHALL display help information when requested.

#### Scenario: Long help flag
- **WHEN** user specifies `--help`
- **THEN** usage information is displayed and program exits

#### Scenario: Short help flag
- **WHEN** user specifies `-h`
- **THEN** usage information is displayed and program exits

#### Scenario: Help content
- **WHEN** help is displayed
- **THEN** it includes:
  - Program description
  - Usage syntax
  - All available options with descriptions
  - Supported operations
  - Examples

#### Scenario: Help examples
- **WHEN** help is displayed
- **THEN** examples include:
  - Basic arithmetic
  - Complex numbers
  - Trigonometry
  - Percentages
  - Equation solving
  - Linear systems

### Requirement: Version Flag
The system SHALL display version information when requested.

#### Scenario: Long version flag
- **WHEN** user specifies `--version`
- **THEN** version number is displayed and program exits

#### Scenario: Short version flag
- **WHEN** user specifies `-v`
- **THEN** version number is displayed and program exits

#### Scenario: Version format
- **WHEN** version is displayed
- **THEN** it is in format `"1.0.0"` or similar semantic version

### Requirement: Output Mode Flags
The system SHALL support flags to control output formatting.

#### Scenario: Pretty flag (auto-detect)
- **WHEN** user specifies `--pretty` or `-p`
- **THEN** output uses best available format (ASCII → Unicode → LaTeX)

#### Scenario: Unicode flag
- **WHEN** user specifies `--unicode` or `-u`
- **THEN** output uses Unicode symbols exclusively

#### Scenario: ASCII flag
- **WHEN** user specifies `--ascii` or `-a`
- **THEN** output uses ASCII characters only

#### Scenario: LaTeX flag
- **WHEN** user specifies `--latex` or `-l`
- **THEN** output is LaTeX source code

#### Scenario: Flag priority
- **WHEN** multiple output flags are specified
- **THEN** the last flag takes precedence

#### Scenario: Default output mode
- **WHEN** no output flags are specified
- **THEN** ASCII mode is used

### Requirement: Interactive Mode
The system SHALL provide an interactive REPL for evaluating expressions.

#### Scenario: Enter interactive mode
- **WHEN** no expression argument is provided
- **THEN** prompt is displayed and user can input expressions

#### Scenario: Display prompt
- **WHEN** in interactive mode
- **THEN** prompt (`>>> ` or pretty variant) is displayed before each input

#### Scenario: Evaluate expression
- **WHEN** user enters expression in interactive mode
- **THEN** expression is evaluated and result is displayed

#### Scenario: Empty input handling
- **WHEN** user enters empty line in interactive mode
- **THEN** prompt is displayed again without evaluation

#### Scenario: Continue after evaluation
- **WHEN** expression is evaluated in interactive mode
- **THEN** prompt is displayed again for next input

#### Scenario: Exit interactive mode
- **WHEN** user sends EOF (Ctrl+D)
- **THEN** interactive mode exits and program terminates

#### Scenario: Pretty prompt in Unicode mode
- **WHEN** in Unicode interactive mode
- **THEN** prompt uses Unicode symbol

#### Scenario: Standard prompt in ASCII mode
- **WHEN** in ASCII interactive mode
- **THEN** prompt is `">>> "`

### Requirement: Exit Codes
The system SHALL return appropriate exit codes to indicate success or failure.

#### Scenario: Successful evaluation
- **WHEN** expression is evaluated successfully
- **THEN** exit code is `0`

#### Scenario: Error in evaluation
- **WHEN** expression evaluation produces an error
- **THEN** exit code is `1`

#### Scenario: Help flag
- **WHEN** `--help` or `-h` is specified
- **THEN** exit code is `0`

#### Scenario: Version flag
- **WHEN** `--version` or `-v` is specified
- **THEN** exit code is `0`

#### Scenario: Interactive mode exit
- **WHEN** interactive mode is exited with Ctrl+D
- **THEN** exit code is `0`

### Requirement: Error Output
The system SHALL display error messages to stdout with clear formatting.

#### Scenario: Error message prefix
- **WHEN** an error occurs
- **THEN** message starts with `"Error:"`

#### Scenario: Expression evaluation error
- **WHEN** expression cannot be evaluated
- **THEN** error message describes the problem and program exits with code 1

#### Scenario: Interactive mode error
- **WHEN** error occurs in interactive mode
- **THEN** error message is displayed and prompt continues

#### Scenario: Invalid expression error
- **WHEN** expression has invalid syntax
- **THEN** error message indicates syntax issue

#### Scenario: Equation solving error
- **WHEN** equation cannot be solved
- **THEN** error message describes why solving failed

### Requirement: Standard Input Handling
The system SHALL handle standard input for expression evaluation.

#### Scenario: Read from stdin in non-interactive mode
- **WHEN** expression is piped via stdin
- **THEN** expression is read and evaluated

#### Scenario: Read from stdin in interactive mode
- **WHEN** no arguments provided
- **THEN** interactive mode reads from stdin line by line

#### Scenario: Handle empty stdin
- **WHEN** stdin is empty
- **THEN** program exits gracefully

#### Scenario: Handle EOF during interactive mode
- **WHEN** EOF is received in interactive mode
- **THEN** program exits gracefully

### Requirement: Expression Argument Handling
The system SHALL handle expression arguments correctly.

#### Scenario: Single expression argument
- **WHEN** providing one expression
- **THEN** it is evaluated and result is printed

#### Scenario: Expression with flags
- **WHEN** providing expression after flags
- **THEN** expression is evaluated with flag configuration

#### Scenario: Expression with spaces
- **WHEN** providing `"3 + 5 * 2"` as argument
- **THEN** entire string is treated as expression

#### Scenario: Expression with special characters
- **WHEN** providing expression with parentheses, operators, functions
- **THEN** special characters are preserved in expression

#### Scenario: Expression with Unicode
- **WHEN** providing expression with Unicode characters
- **THEN** Unicode is preserved and handled correctly

### Requirement: Flag Validation
The system SHALL validate command-line flags and reject invalid combinations.

#### Scenario: Unknown flag
- **WHEN** providing unknown flag
- **THEN** error message indicates invalid option

#### Scenario: Invalid flag combination
- **WHEN** providing mutually exclusive flags (if any exist)
- **THEN** error message indicates conflict

#### Scenario: Flag without value (if required)
- **WHEN** providing flag that requires value but none is given
- **THEN** error message indicates missing value

#### Scenario: Flag after expression
- **WHEN** providing flags after expression argument
- **THEN** they are treated as expression (not flags)

### Requirement: Locale Handling
The system SHALL handle locale settings for consistent parsing.

#### Scenario: Default locale parsing
- **WHEN** parsing expressions
- **THEN** decimal point is `"."`

#### Scenario: Locale override
- **WHEN** `LC_ALL=C` is set
- **THEN** decimal point is `"."`

#### Scenario: Comma decimal handling
- **WHEN** locale uses comma for decimals
- **THEN** user is warned to set `LC_ALL=C` if needed

#### Scenario: Number formatting
- **WHEN** formatting numbers for output
- **THEN** decimal point is `"."` regardless of locale

### Requirement: Output Display
The system SHALL display evaluation results appropriately.

#### Scenario: Single-line output
- **WHEN** evaluating expression from command line
- **THEN** result is displayed on single line

#### Scenario: Newline after result
- **WHEN** result is displayed
- **THEN** newline follows the result

#### Scenario: Interactive mode output
- **WHEN** result is displayed in interactive mode
- **THEN** it is followed by prompt for next input

#### Scenario: No output on flag-only
- **WHEN** only flags (no expression) are provided
- **THEN** no evaluation result is displayed

#### Scenario: Pretty output
- **WHEN** pretty mode is enabled
- **THEN** result is formatted according to terminal capabilities

### Requirement: Command Line Interface Design
The system SHALL follow common CLI conventions.

#### Scenario: Flag naming
- **WHEN** flags are used
- **THEN** long form uses double dash (`--flag`), short form uses single dash (`-f`)

#### Scenario: Flag documentation
- **WHEN** help is displayed
- **THEN** each flag shows long form, short form, and description

#### Scenario: Program name
- **WHEN** help is displayed
- **THEN** it uses the program name (e.g., `"calculator"`)

#### Scenario: Usage syntax
- **WHEN** help is displayed
- **THEN** usage shows: `"calculator [OPTIONS] \"expression\""`
