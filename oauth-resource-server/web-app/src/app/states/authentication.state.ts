import {Action, State, StateContext} from '@ngxs/store';
import {Injectable} from '@angular/core';

export class TokenReceivedAction {
  static readonly type = 'TokenReceived';

  constructor(public token: string) {
  }
}

export class LogoutAction {
  static readonly type = 'Logout';
}

export interface AuthenticationModel {
  token: string;
  loggedIn: boolean;
}

@State<AuthenticationModel>({
  name: 'authentication'
})
@Injectable({
  providedIn: 'root'
})
export class AuthenticationState {

  @Action(TokenReceivedAction)
  tokenReceived(ctx: StateContext<AuthenticationModel>, action: TokenReceivedAction): void {
    ctx.setState({
      token: action.token,
      loggedIn: true
    });
  }

  @Action(LogoutAction)
  logout(ctx: StateContext<AuthenticationModel>): void {
    ctx.setState({
      token: undefined,
      loggedIn: false
    });
  }
}
