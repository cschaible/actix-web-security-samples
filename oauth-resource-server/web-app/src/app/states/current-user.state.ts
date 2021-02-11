import {Action, Selector, State, StateContext} from '@ngxs/store';
import {SaveUserResource} from '../resources/user/save-user-resource';
import {UserService} from '../services/user.service';
import {Injectable} from '@angular/core';
import {RegisterUserResource} from '../resources/user/register-user-resource';
import {BaseUser} from '../model/user/base-user';
import {User} from '../model/user/user';
import {NoUser} from '../model/user/no-user';
import {OAuthService} from 'angular-oauth2-oidc';
import {UserUtils} from './utils';

export class LoadUserAction {
  static readonly type = 'LoadUser';
}

export class RegisterUserAction {
  static readonly type = 'RegisterUser';

  constructor(public firstName: string, public lastName: string) {
  }
}

export class UpdateCurrentUserAction {
  static readonly type = 'UpdateCurrentUser';

  constructor(public firstName: string, public lastName: string) {
  }
}

export class DeleteCurrentUserAction {
  static readonly type = 'DeleteCurrentUser';
}

export class LogoutUserAction {
  static readonly type = 'LogoutUser';
}

@State<BaseUser>({
  name: 'currentUser'
})
@Injectable({
  providedIn: 'root'
})
export class CurrentUserState {

  constructor(private userService: UserService, private oauthService: OAuthService) {
  }

  @Selector()
  static userName(state: BaseUser): string {
    if (state instanceof User) {
      const user = state as User;
      return user.firstName + ' ' + user.lastName;
    }
    return '';
  }

  @Action(LoadUserAction)
  loadUser(ctx: StateContext<BaseUser>): void {
    // Load current user details via REST call and update the state
    this.userService.getCurrentUser().subscribe(usr => ctx.setState(UserUtils.mapUser(usr)));
  }

  @Action(RegisterUserAction)
  registerUser(ctx: StateContext<BaseUser>, action: RegisterUserAction): void {

    // Create REST resource with data from action
    const resource = new RegisterUserResource();
    resource.firstName = action.firstName;
    resource.lastName = action.lastName;

    // Save changes and update the state with returned data
    this.userService.registerCurrentUser(resource).subscribe(usr => ctx.setState(UserUtils.mapUser(usr)));
  }

  @Action(UpdateCurrentUserAction)
  updateUser(ctx: StateContext<BaseUser>, action: UpdateCurrentUserAction): void {

    // Create REST resource with data from action
    const resource = new SaveUserResource();
    resource.firstName = action.firstName;
    resource.lastName = action.lastName;
    const user = ctx.getState() as User;
    if (user.type === 'User') {
      resource.version = user.version;
    }

    // Save changes and update the state with returned data
    this.userService.updateCurrentUser(resource).subscribe(usr => ctx.setState(UserUtils.mapUser(usr)));
  }

  @Action(DeleteCurrentUserAction)
  deleteUser(): void {
    this.userService.deleteCurrentUser().subscribe(_ => {
      this.oauthService.logOut();
    });
  }

  @Action(LogoutUserAction)
  logoutUser(ctx: StateContext<BaseUser>): void {
    ctx.setState(new NoUser());
  }
}
