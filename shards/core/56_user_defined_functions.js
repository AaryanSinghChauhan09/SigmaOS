/**
 * SigmaOS User-Defined Functions Shard
 * Allows users to define, store, and execute custom functions
 */

class UserDefinedFunction {
    constructor(name, code, description = '') {
        this.name = name;
        this.code = code;
        this.description = description;
        this.createdAt = new Date();
        this.executionCount = 0;
    }

    execute(...args) {
        this.executionCount++;
        console.log(`Σ://UDF> Executing function '${this.name}'...');
        try {
            const func = new Function(...args, this.code);
            return func(...args);
        } catch (error) {
            console.error(`Σ://ERROR> UDF execution failed: ' + error);
            throw error;
        }
    }

    toString() {
        return `[UserDefinedFunction: ${this.name}]';
    }
}

class SigmaUDFManager {
    constructor() {
        this.shardId = 'S56_UserDefinedFunctions';
        this.functions = new Map();
        console.log(`Σ://INIT> ${this.shardId} Initializing user-defined functions manager...');
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            console.log(`Σ://UDF> ${this.shardId} Online. User-defined functions manager active.');
        });
    }

    defineFunction(name, code, description = '') {
        if (this.functions.has(name)) {
            console.warn(`Σ://UDF> Function '${name}' already exists. Overwriting...');
        }
        const udf = new UserDefinedFunction(name, code, description);
        this.functions.set(name, udf);
        console.log(`Σ://UDF> Function '${name}' defined successfully.');
        return udf;
    }

    executeFunction(name, ...args) {
        const udf = this.functions.get(name);
        if (!udf) {
            throw new Error(`Function '${name}' not found.');
        }
        return udf.execute(...args);
    }

    deleteFunction(name) {
        if (this.functions.delete(name)) {
            console.log(`Σ://UDF> Function '${name}' deleted successfully.');
        } else {
            console.warn(`Σ://UDF> Function '${name}' not found.');
        }
    }

    listFunctions() {
        return Array.from(this.functions.values());
    }

    getFunction(name) {
        return this.functions.get(name);
    }

    saveFunctions() {
        const data = JSON.stringify(Array.from(this.functions.entries()));
        localStorage.setItem('sigma_udfs', data);
        console.log(`Σ://UDF> Functions saved to local storage.');
    }

    loadFunctions() {
        const data = localStorage.getItem('sigma_udfs');
        if (data) {
            const entries = JSON.parse(data);
            this.functions = new Map(entries.map(([name, udfData]) => {
                const udf = new UserDefinedFunction(name, udfData.code, udfData.description);
                udf.createdAt = new Date(udfData.createdAt);
                udf.executionCount = udfData.executionCount;
                return [name, udf];
            });
            console.log(`Σ://UDF> Functions loaded from local storage.');
        }
    }
}

window.SigmaUDF = new SigmaUDFManager();
