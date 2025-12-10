# LIMIT-HUB: Open-Source Knowledge Hub with Governance

Open-source hub for SARS-CoV-2 knowledge sharing with governance rules, validation, and REST API.

## Features

### Governance Rules
- Minimum confidence thresholds
- Provenance count requirements
- Allowed source validation
- Quality score thresholds
- Review requirements

### REST API
- Health check endpoint
- Submission endpoint with validation
- List all submissions
- Get submission by ID
- Validate submission without storing

### Validation
- Comprehensive error reporting
- Warning generation
- Review requirement flagging
- Custom rule support

## API Endpoints

```
GET  /health              - Health check
POST /submit              - Submit data with validation
GET  /submissions         - List all submissions
GET  /submissions/:id     - Get specific submission
POST /validate            - Validate without storing
```

## Usage

### Start Server

```bash
cargo run --bin limit-hub
```

Server runs on `http://0.0.0.0:3000`

### Submit Data

```rust
use limit_hub::governance::{GovernanceRules, Submission};

let rules = GovernanceRules::default_rules();
let submission = Submission {
    id: "sub-001".to_string(),
    content: "New variant data".to_string(),
    confidence: 0.85,
    provenance: vec!["PubMed".to_string()],
    quality_score: 0.9,
    metadata: HashMap::new(),
};

let validation = rules.validate_submission(&submission);
```

## Integration

Provides centralized hub for SARS-CoV-2 knowledge graph contributions with quality control.
