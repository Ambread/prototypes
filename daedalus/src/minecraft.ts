export {};

const command = () => new Command();

class Command {
    constructor(private output: string[] = []) {}

    say(message: string) {
        this.output.push('say', message);
        return this.output.join(' ');
    }

    get execute() {
        this.output.push('execute');
        return new Execute(this.output);
    }
}

class Execute {
    constructor(private output: string[]) {}

    as(selector: string) {
        this.output.push('as', selector);
        return this;
    }

    get run() {
        return new Command(this.output);
    }
}

console.log(command().execute.as('foo').run.say('hi'));
