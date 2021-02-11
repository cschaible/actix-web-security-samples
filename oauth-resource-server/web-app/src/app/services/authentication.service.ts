import {Injectable} from '@angular/core';
import {AuthConfig, OAuthService} from 'angular-oauth2-oidc';
import {environment} from '../../environments/environment';
import {JwksValidationHandler} from 'angular-oauth2-oidc-jwks';
import {LogoutAction, TokenReceivedAction} from '../states/authentication.state';
import {LoadUserAction, LogoutUserAction} from '../states/current-user.state';
import {Router} from '@angular/router';
import {Store} from '@ngxs/store';
import {BaseUser} from '../model/user/base-user';
import {User} from '../model/user/user';
import {UnregisteredUser} from '../model/user/unregistered-user';

@Injectable({
  providedIn: 'root'
})
export class AuthenticationService {

  private authConfig: AuthConfig = {
    // Url of the Identity Provider
    issuer: environment.idp + '/auth/realms/app',

    // URL of the SPA to redirect the user to after login
    redirectUri: window.location.origin,

    tokenEndpoint: environment.idp + '/auth/realms/app/protocol/openid-connect/token',

    // The SPA's id. The SPA is registered with this id at the auth-server
    // clientId: 'server.code',
    clientId: 'web-app',

    responseType: 'code',

    // set the scope for the permissions the client should request
    // The first four are defined by OIDC.
    // Important: Request offline_access to get a refresh token
    scope: 'openid profile email offline_access',

    showDebugInformation: false,
  };

  constructor(private oauthService: OAuthService, private store: Store, private router: Router) {
    this.oauthService.configure(this.authConfig);
    this.oauthService.tokenValidationHandler = new JwksValidationHandler();
    this.oauthService.loadDiscoveryDocument(environment.idp + '/auth/realms/app/.well-known/openid-configuration').then(_ => {
      return;
    });
    this.oauthService.loadDiscoveryDocumentAndTryLogin().then(_ => {
      return;
    });
    this.oauthService.setupAutomaticSilentRefresh();

    this.oauthService.events
      .subscribe(e => {
        if (e.type === 'token_received') {
          this.store.dispatch(new TokenReceivedAction(this.oauthService.getAccessToken())).subscribe(ignored => this.load_user());
        } else if (e.type === 'logout' || e.type === 'session_terminated') {
          this.store.dispatch(new LogoutAction()).subscribe(ignored => this.store.dispatch(new LogoutUserAction()));
        }
      });
  }

  public login(): void {
    this.oauthService.initCodeFlow();
  }

  public logout(): void {
    this.oauthService.logOut();
  }

  private load_user(): void {
    this.store.dispatch(new LoadUserAction()).subscribe(_ => {
      this.store.select(state => state.currentUser).subscribe(currentUser => {
        if (Object.keys(currentUser).length === 0 && currentUser.constructor === Object) {
          return;
        }
        const user = currentUser as BaseUser;

        if (user instanceof User) {
          if (this.router.isActive('/', true)) {
            this.router.navigate(['/']);
          }
        } else if (user instanceof UnregisteredUser) {
          this.router.navigate(['/register']);
        }
        // do nothing in case user is instance of NoUser
      });
    });
  }
}
