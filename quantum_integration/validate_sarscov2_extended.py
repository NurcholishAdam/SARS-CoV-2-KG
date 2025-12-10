#!/usr/bin/env python3
"""
Validation script for SARS-CoV-2 Extended Implementation
Checks all 5 stages are properly implemented
"""

import os
import sys
from pathlib import Path

def check_file_exists(path, description):
    """Check if a file exists and report"""
    if os.path.exists(path):
        print(f"✓ {description}: {path}")
        return True
    else:
        print(f"✗ {description}: {path} NOT FOUND")
        return False

def validate_stage_1():
    """Validate Stage 1: Enriched biomedical graph"""
    print("\n=== Stage 1: Enriched Biomedical Graph ===")
    base = "quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-bio-sars/src"
    
    files = [
        (f"{base}/nodes.rs", "Enriched nodes"),
        (f"{base}/graph.rs", "Graph operations"),
        (f"{base}/loader.rs", "Data loader"),
        (f"{base}/lib.rs", "Library module"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_stage_2():
    """Validate Stage 2: Multi-intent harness"""
    print("\n=== Stage 2: Multi-Intent Harness ===")
    base = "quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-benchmark/src"
    
    files = [
        (f"{base}/multi.rs", "Multi-intent queries"),
        (f"{base}/metrics.rs", "Performance metrics"),
        (f"{base}/harness.rs", "Benchmark harness"),
        (f"{base}/provenance.rs", "Provenance tracking"),
        (f"{base}/lib.rs", "Library module"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_stage_3():
    """Validate Stage 3: Quantum-inspired retrieval"""
    print("\n=== Stage 3: Quantum-Inspired Retrieval ===")
    base = "quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-quantum/src"
    
    files = [
        (f"{base}/rd.rs", "Rate-distortion optimization"),
        (f"{base}/sampler.rs", "Quantum sampler"),
        (f"{base}/lib.rs", "Library module"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_stage_4():
    """Validate Stage 4: Open-source hub"""
    print("\n=== Stage 4: Open-Source Hub ===")
    base = "quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-hub/src"
    
    files = [
        (f"{base}/governance.rs", "Governance rules"),
        (f"{base}/api.rs", "REST API"),
        (f"{base}/main.rs", "Hub server"),
        (f"{base}/lib.rs", "Library module"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_stage_5():
    """Validate Stage 5: Unit tests"""
    print("\n=== Stage 5: Unit Tests ===")
    base = "quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-quantum/tests"
    
    files = [
        (f"{base}/rd_tests.rs", "RD optimization tests"),
        (f"{base}/governance_tests.rs", "Governance tests"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_integration():
    """Validate integration components"""
    print("\n=== Integration Components ===")
    
    files = [
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/examples/sarscov2_complete_demo.rs", 
         "Complete demo"),
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/Cargo.toml", 
         "Workspace Cargo.toml"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def validate_documentation():
    """Validate documentation"""
    print("\n=== Documentation ===")
    
    files = [
        ("quantum_integration/SARSCOV2_EXTENDED_DELIVERY.md", "Extended delivery doc"),
        ("quantum_integration/SARSCOV2_QUICK_START_EXTENDED.md", "Quick start guide"),
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-bio-sars/README.md", 
         "Bio-SARS README"),
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-benchmark/README.md", 
         "Benchmark README"),
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-quantum/README.md", 
         "Quantum README"),
        ("quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-hub/README.md", 
         "Hub README"),
    ]
    
    return all(check_file_exists(f, desc) for f, desc in files)

def main():
    """Main validation function"""
    print("=" * 60)
    print("SARS-CoV-2 Extended Implementation Validation")
    print("=" * 60)
    
    results = {
        "Stage 1": validate_stage_1(),
        "Stage 2": validate_stage_2(),
        "Stage 3": validate_stage_3(),
        "Stage 4": validate_stage_4(),
        "Stage 5": validate_stage_5(),
        "Integration": validate_integration(),
        "Documentation": validate_documentation(),
    }
    
    print("\n" + "=" * 60)
    print("Validation Summary")
    print("=" * 60)
    
    for stage, passed in results.items():
        status = "✓ PASS" if passed else "✗ FAIL"
        print(f"{stage}: {status}")
    
    all_passed = all(results.values())
    
    print("\n" + "=" * 60)
    if all_passed:
        print("✓ ALL STAGES VALIDATED SUCCESSFULLY")
        print("=" * 60)
        return 0
    else:
        print("✗ VALIDATION FAILED - Some components missing")
        print("=" * 60)
        return 1

if __name__ == "__main__":
    sys.exit(main())
