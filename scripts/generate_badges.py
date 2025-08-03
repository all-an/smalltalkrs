#!/usr/bin/env python3
"""
Badge generation script for SmalltalkRS
Generates local badges from build and test data
"""

import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, Any

def run_command(cmd: str) -> tuple[int, str, str]:
    """Run a shell command and return exit code, stdout, stderr"""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.returncode, result.stdout, result.stderr

def get_cargo_version() -> str:
    """Get the current cargo package version"""
    try:
        _, stdout, _ = run_command("cargo pkgid")
        # Extract version from output like "smalltalkrs#0.1.0"
        match = re.search(r'#(.+)$', stdout.strip())
        return match.group(1) if match else "unknown"
    except Exception:
        return "unknown"

def generate_build_status() -> str:
    """Generate build status by running cargo check"""
    print("🔨 Checking build status...")
    exit_code, _, stderr = run_command("cargo check --quiet")
    
    if exit_code == 0:
        return "passing"
    else:
        print(f"❌ Build failed: {stderr}")
        return "failing"

def run_tests() -> Dict[str, Any]:
    """Run tests and extract results"""
    print("🧪 Running tests...")
    exit_code, stdout, stderr = run_command("cargo test --quiet 2>&1")
    
    # Parse test output
    test_results = {
        "status": "passing" if exit_code == 0 else "failing",
        "passed": 0,
        "failed": 0,
        "total": 0
    }
    
    # Extract test summary from output
    for line in stdout.split('\n'):
        if "test result:" in line:
            # Format: "test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
            match = re.search(r'(\d+) passed; (\d+) failed', line)
            if match:
                test_results["passed"] = int(match.group(1))
                test_results["failed"] = int(match.group(2))
                test_results["total"] = test_results["passed"] + test_results["failed"]
    
    return test_results

def generate_coverage() -> float:
    """Get coverage percentage from existing reports or generate new one"""
    print("📊 Checking coverage data...")
    
    # Try to read from existing JSON report first
    json_files = Path(".").glob("*.json")
    for json_file in json_files:
        try:
            with open(json_file) as f:
                data = json.load(f)
                if 'coverage' in data:
                    coverage = data['coverage']
                    print(f"📈 Found coverage in {json_file}: {coverage:.1f}%")
                    return coverage
        except (json.JSONDecodeError, KeyError, FileNotFoundError):
            continue
    
    # Try to parse from tarpaulin JSON report in coverage directory
    coverage_dir = Path("coverage")
    if coverage_dir.exists():
        for file in coverage_dir.glob("*.json"):
            try:
                with open(file) as f:
                    data = json.load(f)
                    if 'coverage' in data:
                        coverage = data['coverage']
                        print(f"📈 Found coverage in {file}: {coverage:.1f}%")
                        return coverage
            except (json.JSONDecodeError, KeyError):
                continue
    
    # Quick tarpaulin run with just percentage output
    print("📊 Running quick coverage check...")
    exit_code, stdout, stderr = run_command("cargo tarpaulin --skip-clean --engine llvm")
    
    if exit_code == 0:
        # Parse percentage from text output
        for line in stdout.split('\n'):
            if '%' in line and 'coverage' in line.lower():
                match = re.search(r'(\d+\.?\d*)%', line)
                if match:
                    coverage = float(match.group(1))
                    print(f"📈 Coverage: {coverage:.1f}%")
                    return coverage
    
    print("⚠️ Could not determine coverage, using 0%")
    return 0.0

def create_badge_url(label: str, message: str, color: str) -> str:
    """Create a shields.io badge URL"""
    # URL encode spaces and special characters
    label = label.replace(" ", "%20")
    message = message.replace(" ", "%20")
    return f"https://img.shields.io/badge/{label}-{message}-{color}"

def generate_badges() -> Dict[str, str]:
    """Generate all badges and return URLs"""
    badges = {}
    
    # Version badge
    version = get_cargo_version()
    badges["version"] = create_badge_url("version", f"v{version}", "blue")
    
    # Build status badge
    build_status = generate_build_status()
    build_color = "brightgreen" if build_status == "passing" else "red"
    badges["build"] = create_badge_url("build", build_status, build_color)
    
    # Test results badge
    test_results = run_tests()
    test_message = f"{test_results['passed']}%20passing" if test_results['total'] > 0 else "0%20tests"
    test_color = "brightgreen" if test_results["status"] == "passing" else "red"
    badges["tests"] = create_badge_url("tests", test_message, test_color)
    
    # Coverage badge
    coverage = generate_coverage()
    coverage_color = "brightgreen" if coverage >= 90 else "yellow" if coverage >= 70 else "red"
    badges["coverage"] = create_badge_url("coverage", f"{coverage:.1f}%25", coverage_color)
    
    return badges

def update_readme(badges: Dict[str, str]):
    """Update README.md with new badge URLs"""
    readme_path = Path("README.md")
    if not readme_path.exists():
        print("❌ README.md not found")
        return
    
    content = readme_path.read_text()
    
    # Update badge URLs - more specific patterns for SmalltalkRS
    replacements = [
        (r'\[!\[Coverage\].*?\]\([^)]*\)', f'[![Coverage]({badges["coverage"]})](#)'),
        (r'\[!\[Build Status\].*?\]\([^)]*\)', f'[![Build Status]({badges["build"]})](#)'),
        (r'\[!\[Tests\].*?\]\([^)]*\)', f'[![Tests]({badges["tests"]})](#)'),
        (r'\[!\[Version\].*?\]\([^)]*\)', f'[![Version]({badges["version"]})](#)'),
    ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)
    
    readme_path.write_text(content)
    print("✅ README.md updated with new badges")

def save_badge_data(badges: Dict[str, str]):
    """Save badge data to file for reference"""
    badge_data = {
        "generated_at": subprocess.check_output(["date", "-Iseconds"]).decode().strip(),
        "badges": badges
    }
    
    Path("badges").mkdir(exist_ok=True)
    with open("badges/data.json", "w") as f:
        json.dump(badge_data, f, indent=2)
    
    print("💾 Badge data saved to badges/data.json")

def main():
    """Main function"""
    print("🎯 SmalltalkRS Badge Generator")
    print("=" * 40)
    
    try:
        badges = generate_badges()
        save_badge_data(badges)
        update_readme(badges)
        
        print("\n✅ All badges generated successfully!")
        print("\nGenerated badges:")
        for name, url in badges.items():
            print(f"  {name}: {url}")
            
    except Exception as e:
        print(f"❌ Error generating badges: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()