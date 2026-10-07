// SPDX-License-Identifier: MIT
// Exercise the inline installation preview flow without any disk access or browser dependencies.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const root = path.join(__dirname, '..');
const desktop = fs.readFileSync(path.join(root, 'index.html'), 'utf8');
const html = fs.readFileSync(path.join(root, 'web_ui/index.html'), 'utf8');
assert.match(desktop, /<iframe src="web_ui\/index\.html"/);
assert.ok(fs.existsSync(path.join(root, 'web_ui/styles/style.css')));
assert.doesNotMatch(html, /\/dev\/sda|Detected OS:|successfully written to disk|Rebooting node|startInstallationSimulation/);
assert.match(html, /No disk was detected or modified, no OS was installed/);

let focused = null;
function element(id) {
    const classes = new Set(id === 'panel-1' ? ['active'] : []);
    const attributes = {};
    return {
        id, style: {}, value: id === 'hostname-input' ? 'sigmaos-node' : '', checked: false,
        disabled: id === 'btn-back', textContent: '', tabIndex: 0,
        classList: {
            add: name => classes.add(name), remove: name => classes.delete(name),
            contains: name => classes.has(name),
        },
        focus() { focused = this; }, click() { this.onClick(); },
        setAttribute(name, value) { attributes[name] = value; },
        getAttribute(name) { return attributes[name]; },
        removeAttribute(name) { delete attributes[name]; },
    };
}
const elements = Object.fromEntries(
    [...html.matchAll(/\bid="([^"]+)"/g)].map(([, id]) => [id, element(id)]),
);
const disks = [element('sample-ssd'), element('sample-usb')];
const modes = [element('dual'), element('clean'), element('custom')];
for (const [index, card] of disks.entries()) {
    card.onClick = () => context.selectDisk(card, ['sample-ssd', 'sample-usb'][index]);
    card.closest = () => ({ querySelectorAll: () => disks });
}
for (const [index, card] of modes.entries()) {
    card.onClick = () => context.selectInstallMode(card, ['dual', 'clean', 'custom'][index]);
    card.closest = () => ({ querySelectorAll: () => modes });
}
const document = {
    getElementById: id => elements[id],
    querySelector: selector => selector === '.disk-card' ? disks[0] : modes[0],
    querySelectorAll: selector => selector === '.disk-card' ? disks : selector === '.mode-card' ? modes : [],
};
const source = html.match(/<script>([\s\S]*?)<\/script>/)[1];
const context = vm.createContext({ document });
vm.runInContext(source, context);
const step = () => Number(vm.runInContext('currentStep', context));

context.nextStep();
assert.equal(step(), 1, 'must acknowledge preview before advancing');
elements['accept-license'].checked = true;
context.nextStep();
context.nextStep();
assert.equal(step(), 2, 'no implicit disk selection');
assert.equal(elements['disk-error'].style.display, 'block');

const keyEvent = { key: ' ', preventDefault() { this.prevented = true; } };
context.handleCardKeydown(keyEvent, disks[0]);
assert.equal(keyEvent.prevented, true);
assert.equal(disks[0].getAttribute('aria-checked'), 'true');
assert.equal(disks[0].tabIndex, 0);
const arrow = { key: 'ArrowDown', preventDefault() { this.prevented = true; } };
context.handleCardKeydown(arrow, disks[0]);
assert.equal(arrow.prevented, true);
assert.equal(focused, disks[1]);
assert.equal(disks[1].getAttribute('aria-checked'), 'true');
assert.equal(disks[0].tabIndex, -1);
context.handleCardKeydown({ key: 'ArrowUp', preventDefault() {} }, disks[1]);
assert.equal(disks[0].getAttribute('aria-checked'), 'true');
context.nextStep();
context.nextStep();
assert.equal(step(), 3, 'no implicit partition mode selection');
modes[1].click();
assert.equal(modes[1].getAttribute('aria-checked'), 'true');
context.nextStep();
context.nextStep();
assert.equal(step(), 5);
assert.equal(elements['review-device'].textContent, 'Example internal SSD (512 GB)');
assert.match(elements['review-mode'].textContent, /Erase entire disk/);
assert.match(elements['review-warning'].textContent, /permanently delete all data/);
context.nextStep();
assert.equal(step(), 5, 'erase preview needs explicit review acknowledgement');
assert.equal(focused, elements['confirm-preview']);
elements['confirm-preview'].checked = true;
context.nextStep();
assert.equal(step(), 6);
assert.equal(elements['btn-next'].disabled, true);
assert.equal(elements['btn-next'].textContent, 'Done');
console.log('Installation preview flow and truthful UI checks passed');
