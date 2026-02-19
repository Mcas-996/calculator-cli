# core Specification

## Purpose
TBD - created by archiving change fix-fraction-partialord. Update Purpose after archive.
## Requirements
### Requirement: Fraction Numeric Comparison
The Fraction struct SHALL provide numeric comparison operations using the PartialOrd trait.

#### Scenario: Compare two fractions
- **WHEN** comparing two Fraction instances
- **THEN** the comparison SHALL be based on their underlying Rational64 values
- **THEN** the comparison SHALL correctly handle positive and negative fractions
- **THEN** the comparison SHALL correctly handle fractions with different denominators

#### Scenario: Order fractions in a collection
- **WHEN** placing Fraction instances in an ordered collection
- **THEN** the ordering SHALL follow standard mathematical fractions ordering
- **THEN** no compilation errors SHALL occur due to trait conflicts

#### Scenario: Use fraction in algorithms requiring comparisons
- **WHEN** using Fraction instances with algorithms requiring comparison capabilities
- **THEN** the PartialOrd implementation SHALL work seamlessly
- **THEN** no manual trait implementations SHALL conflict with derived ones

