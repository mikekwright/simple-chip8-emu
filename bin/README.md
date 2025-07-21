# Scripts Directory

This directory contains helper scripts for building, testing, and working with the CHIP-8 emulator.

## Scripts

### `build_help.sh`

Displays comprehensive build options and command examples for all feature combinations.

**Usage:**

```bash
bash bin/build_help.sh
```

**What it shows:**

- Default build (both GUI and TUI)
- TUI-only build commands
- GUI-only build commands  
- Release build options
- Binary size benefits explanation

### `verify_builds.sh`

Tests all build combinations to ensure they compile correctly.

**Usage:**

```bash
bash bin/verify_builds.sh
```

**What it tests:**

- TUI-only build verification
- GUI-only build verification
- Default build (both features) verification
- Reports success/failure for each combination

### `test_all.sh`

Comprehensive test runner that executes all verification tests and provides usage examples.

**Usage:**

```bash
bash bin/test_all.sh
```

**What it does:**

- Runs the complete build verification suite
- Provides guidance for manual testing
- Shows available commands for each mode

## Quick Reference

```bash
# View build options
bash bin/build_help.sh

# Test all builds
bash bin/verify_builds.sh

# Full test suite
bash bin/test_all.sh
```

All scripts are designed to be run from the project root directory.
