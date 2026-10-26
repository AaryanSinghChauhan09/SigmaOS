# SigmaOS Shell Architecture & Maintenance Guide

## Overview
This document outlines the architecture, expansion rules, exit status policies, interactive prompt behaviors, and maintenance guidelines for the SigmaOS shell ecosystem (`sigma-sh`).

## Maintenance Guidelines
- Ensure deterministic script parsing without partial execution on syntax errors.
- Support standard POSIX expansion order.
- Maintain safety contracts for signal handling and file redirections.
