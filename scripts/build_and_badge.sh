#!/bin/bash
# SmalltalkRS Build and Badge Generation Script
# This script builds the project, runs tests, generates coverage, and updates badges

set -e  # Exit on any error

echo "🦀 SmalltalkRS Build & Badge Generator"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Step 1: Clean previous builds
print_status "Cleaning previous builds..."
cargo clean

# Step 2: Build the project
print_status "Building project..."
if cargo build; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Step 3: Run tests
print_status "Running tests..."
if cargo test; then
    print_success "All tests passed"
else
    print_warning "Some tests failed, but continuing..."
fi

# Step 4: Generate coverage report
print_status "Generating coverage report..."
if command -v cargo-tarpaulin &> /dev/null; then
    if cargo tarpaulin --out Html --output-dir coverage/ --out Json; then
        print_success "Coverage report generated"
    else
        print_warning "Coverage generation failed, but continuing..."
    fi
else
    print_warning "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
fi

# Step 5: Generate badges
print_status "Generating badges..."
if command -v python3 &> /dev/null; then
    if python3 scripts/generate_badges.py; then
        print_success "Badges generated and README updated"
    else
        print_warning "Badge generation failed"
    fi
else
    print_warning "Python3 not found. Badges will not be updated."
fi

# Step 6: Build release version
print_status "Building release version..."
if cargo build --release; then
    print_success "Release build completed"
else
    print_warning "Release build failed"
fi

echo ""
print_success "🎉 Build and badge generation complete!"
echo ""
echo "Generated artifacts:"
echo "  - ./target/debug/smalltalkrs (debug binary)"
echo "  - ./target/release/smalltalkrs (release binary)"
echo "  - ./coverage/ (coverage reports)"
echo "  - ./badges/data.json (badge data)"
echo ""
echo "Next steps:"
echo "  - Run: ./target/release/smalltalkrs"
echo "  - View coverage: open coverage/tarpaulin-report.html"
echo "  - Check badges: cat badges/data.json"