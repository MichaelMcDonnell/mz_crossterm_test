# mz_crossterm_test
Tests that moving the cursor by zero in crossterm does not do anything.

Currently in crossterm version 0.17.7, on terminals that support ANSI escape
codes, the cursor is moved by one when zero is used. This is confusing.