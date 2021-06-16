import { Action } from 'history';

export interface ITransitionDescriptor {
  name: string;
  duration: number;
}

export interface ITransitionFork {
  forward: ITransitionDescriptor;
  backward: ITransitionDescriptor;
}

export interface ITransitionMatch {
  direction: 'forward' | 'backward';
  descriptor: ITransitionDescriptor;
}

export default class TransitionRule {
  constructor(private from: string | null, private to: string, private fork: ITransitionFork) {}

  public match(
    fromRoute: string | null,
    toRoute: string,
    action?: Action,
  ): ITransitionMatch | null {
    if (action !== 'POP' && (!this.from || this.from === fromRoute) && this.to === toRoute) {
      return {
        direction: 'forward',
        descriptor: this.fork.forward,
      };
    }

    if (action !== 'PUSH' && (!this.from || this.from === toRoute) && this.to === fromRoute) {
      return {
        direction: 'backward',
        descriptor: this.fork.backward,
      };
    }

    return null;
  }
}
