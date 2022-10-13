class NFAState {
    transitions: Map<string, NFAState[]>;
    isTerminal: boolean;

    constructor(transitions?: Map<string, NFAState[]>, isTerminal = false) {
        if (transitions === undefined) {
            transitions = new Map();
        }
        this.transitions = transitions;
        this.isTerminal = isTerminal;
    }

    addTransition(ch: string, nextState: NFAState) {
        let nextStates = this.transitions.get(ch);
        if (!nextStates) {
            nextStates = [];
        }
        nextStates.push(nextState);
        this.transitions.set(ch, nextStates);
    }

    match(s: string, idx: number): boolean {
        if (idx > s.length) {
            return false;
        }
        if (idx === s.length && this.isTerminal) {
            return true;
        }
        let nextStates = this.transitions.get(s[idx]);
        if (nextStates) {
            for (const nextState of nextStates) {
                if (nextState.match(s, idx+1)) {
                    return true;
                }
            }
        }
        nextStates = this.transitions.get('.');
        if (nextStates) {
            for (const nextState of nextStates) {
                if (nextState.match(s, idx+1)) {
                    return true;
                }
            }
        }
        nextStates = this.transitions.get('');
        if (nextStates) {
            for (const nextState of nextStates) {
                if (nextState.match(s, idx)) {
                    return true;
                }
            }
        }
        return false;
    }
}

function isMatch(s: string, p: string): boolean {
    const dfa = buildNFA(p);
    return dfa.match(s, 0);
};

function buildNFA(p: string): NFAState {
    let initState = new NFAState();
    let currState = initState;
    for (let idx = 0; idx < p.length; idx++) {
        const currChar = p[idx];
        if (idx + 1 < p.length && p[idx+1] === '*') {
            const nextState = new NFAState();
            nextState.addTransition(currChar, nextState);
            
            currState.addTransition('', nextState);
            currState = nextState;
            idx++;
        } else {
            const nextState = new NFAState();
            currState.addTransition(currChar, nextState);
            currState = nextState;
        }
    }
    currState.isTerminal = true;
    return initState;
}

console.log(isMatch('mississippi', 'mis*is*p*.'));
console.log(isMatch('ab', '.*c'));

export {};