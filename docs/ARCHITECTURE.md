# Architecture

The Learning Content Studio is the LCMS and authoring authority. It owns mutable authoring projects, reusable learning objects, revisions, accessibility variants, localization, rights metadata, review/approval state, and immutable content releases.

It does not own enrollment, learner completion, xAPI statements, psychometric response data, or commercial payment truth.

## Authoring pipeline

Mutable authoring source -> review -> accessibility validation -> rights validation -> approval -> immutable content release -> target-specific publication artifacts.

## Initial modules

- structured content model
- reusable component registry
- asset library
- revision control
- accessibility validator
- localization workflow
- rights management
- release pipeline
- publisher adapters

Inkspan is the preferred reusable authoring surface; learning-specific semantics live in this repository rather than being copied into Inkspan.
