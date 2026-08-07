#!/bin/bash
sed -i 's/fn enroll(&mut self, user_id: usize)/fn enroll(\&mut self, _user_id: usize)/g' src/fingerprint/scanner.rs
sed -i 's/fn verify(&self, template: &dyn FingerprintTemplate)/fn verify(\&self, _template: \&dyn FingerprintTemplate)/g' src/fingerprint/scanner.rs
sed -i 's/fn authenticate(&mut self, fingerprint: &dyn FingerprintTemplate)/fn authenticate(\&mut self, _fingerprint: \&dyn FingerprintTemplate)/g' src/fingerprint/scanner.rs
