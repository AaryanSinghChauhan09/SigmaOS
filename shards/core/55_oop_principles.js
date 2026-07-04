/**
 * SigmaOS OOP Principles Shard
 * Advanced Object-Oriented Programming Principles Implementation
 * Encapsulation, Inheritance, Polymorphism, Abstraction
 */

class SigmaObject {
    constructor(id = 'SigmaObject') {
        this.id = id;
        this._privateData = {};
        this._listeners = {};
        console.log(`Σ://OOP> ${this.id} initialized.`);
    }

    set(key, value) {
        this._privateData[key] = value;
        this.emit('change', { key, value });
    }

    get(key) {
        return this._privateData[key];
    }

    on(event, callback) {
        if (!this._listeners[event]) {
            this._listeners[event] = [];
        }
        this._listeners[event].push(callback);
    }

    emit(event, data) {
        if (this._listeners[event]) {
            this._listeners[event].forEach(callback => callback(data));
        }
    }

    toString() {
        return `[${this.id}]`;
    }
}

class EncapsulationExample extends SigmaObject {
    constructor() {
        super('EncapsulationExample');
        this._secret = 'Encapsulated secret';
    }

    getSecret() {
        return this._secret;
    }

    setSecret(newSecret) {
        this._secret = newSecret;
        console.log(`Σ://OOP> Encapsulation: Secret updated.`);
    }
}

class InheritanceExample extends EncapsulationExample {
    constructor() {
        super();
        this.id = 'InheritanceExample';
        this._additionalData = 'Inherited data';
    }

    getAdditionalData() {
        return this._additionalData;
    }
}

class PolymorphicBase extends SigmaObject {
    constructor() {
        super('PolymorphicBase');
    }

    performAction() {
        console.log(`Σ://OOP> Base action performed.`);
    }
}

class PolymorphicChildA extends PolymorphicBase {
    constructor() {
        super();
        this.id = 'PolymorphicChildA';
    }

    performAction() {
        console.log(`Σ://OOP> Child A action performed.`);
    }
}

class PolymorphicChildB extends PolymorphicBase {
    constructor() {
        super();
        this.id = 'PolymorphicChildB';
    }

    performAction() {
        console.log(`Σ://OOP> Child B action performed.`);
    }
}

class AbstractShape extends SigmaObject {
    constructor() {
        super('AbstractShape');
        if (this.constructor === AbstractShape) {
            throw new Error('Cannot instantiate abstract class AbstractShape directly.');
        }
    }

    getArea() {
        throw new Error('Method getArea() must be implemented.');
    }
}

class Circle extends AbstractShape {
    constructor(radius) {
        super();
        this.id = 'Circle';
        this.radius = radius;
    }

    getArea() {
        return Math.PI * this.radius * this.radius;
    }
}

class Rectangle extends AbstractShape {
    constructor(width, height) {
        super();
        this.id = 'Rectangle';
        this.width = width;
        this.height = height;
    }

    getArea() {
        return this.width * this.height;
    }
}

class SigmaOOPFramework {
    constructor() {
        this.shardId = 'S55_OOPPrinciples';
        this.objects = [];
        console.log(`Σ://INIT> ${this.shardId} Initializing OOP framework...');
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://OOP> ${this.shardId} Online. OOP principles framework active.');
        });
    }

    createObject(type, ...args) {
        let obj;
        switch (type) {
            case 'SigmaObject':
                obj = new SigmaObject(...args);
                break;
            case 'EncapsulationExample':
                obj = new EncapsulationExample(...args);
                break;
            case 'InheritanceExample':
                obj = new InheritanceExample(...args);
                break;
            case 'PolymorphicChildA':
                obj = new PolymorphicChildA(...args);
                break;
            case 'PolymorphicChildB':
                obj = new PolymorphicChildB(...args);
                break;
            case 'Circle':
                obj = new Circle(...args);
                break;
            case 'Rectangle':
                obj = new Rectangle(...args);
                break;
            default:
                throw new Error(`Unknown object type: ' + type);
        }
        this.objects.push(obj);
        console.log(`Σ://OOP> Object ' + type + ' created.');
        return obj;
    }

    demonstratePolymorphism(objects) {
        objects.forEach(obj => {
            if (obj.performAction) {
                obj.performAction();
            } else if (obj.getArea) {
                console.log(`Σ://OOP> ' + obj.id + ' area: ' + obj.getArea());
            }
        });
    }
}

window.SigmaOOP = new SigmaOOPFramework();
