import {Action, State, StateContext} from '@ngxs/store';
import {Injectable} from '@angular/core';
import {UserService} from '../services/user.service';
import {UserUtils} from './utils';
import {User} from '../model/user/user';
import {UserResource} from '../resources/user/user-resource';
import {SaveUserResource} from '../resources/user/save-user-resource';

export class FindUsersAction {
  static readonly type = 'FindUsers';
}

export class UpdateUserAction {
  static readonly type = 'UpdateUser';

  constructor(public updatedUser: User) {
  }
}

export class DeleteUsersAction {
  static readonly type = 'DeleteUser';

  constructor(public user: User) {
  }
}

export class UserModel {
  users: User[] = [];
}

@State<UserModel>({
  name: 'users',
})
@Injectable({
  providedIn: 'root'
})
export class UserState {

  constructor(private userService: UserService) {
  }

  @Action(FindUsersAction)
  findUsers(ctx: StateContext<UserModel>): void {

    this.userService.findAllUsers().subscribe(list => {
      const rawUsers = list.items as UserResource[];
      const users: User[] = UserUtils.mapUsers(rawUsers);
      ctx.setState({users});
    });
  }

  @Action(UpdateUserAction)
  updateUser(ctx: StateContext<UserModel>, action: UpdateUserAction): void {
    const updatedUser = action.updatedUser;
    const updateResource = new SaveUserResource();
    updateResource.version = updatedUser.version;
    updateResource.admin = updatedUser.admin;
    updateResource.firstName = updatedUser.firstName;
    updateResource.lastName = updatedUser.lastName;
    this.userService.updateUser(updateResource, updatedUser.id).subscribe(user => {
      const model = ctx.getState();

      const newModel = new UserModel();
      const newUsers = [...model.users];
      const index = newUsers.findIndex(u => u.id === user.id);
      newUsers[index] = UserUtils.mapUser(user) as User;
      newModel.users = newUsers;
      ctx.setState(newModel);
    });
  }

  @Action(DeleteUsersAction)
  deleteUser(ctx: StateContext<UserModel>, action: DeleteUsersAction): void {
    this.userService.deleteUser(action.user.id).subscribe(_ => {
      const model = ctx.getState();
      ctx.patchState({
        users: [...model.users.filter(u => u.id !== action.user.id)]
      });
    });
  }
}
