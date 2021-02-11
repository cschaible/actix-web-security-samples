import {UserResource} from '../resources/user/user-resource';
import {BaseUser} from '../model/user/base-user';
import {UnregisteredUser} from '../model/user/unregistered-user';
import {User} from '../model/user/user';

export class UserUtils {

  public static mapUser(userResource: UserResource): BaseUser {
    if (Object.keys(userResource).length === 0 && userResource.constructor === Object) {
      return new UnregisteredUser();
    } else {
      const newUser = new User();
      newUser.firstName = userResource.firstName;
      newUser.lastName = userResource.lastName;
      newUser.id = userResource.id;
      newUser.version = userResource.version;
      newUser.admin = userResource.admin;
      return newUser;
    }
  }

  public static mapUsers(userResources: UserResource[]): User[] {
    const users: User[] = new Array(userResources.length);
    let i = 0;
    for (const userResource of userResources) {
      const user = this.mapUser(userResource);
      users[i++] = user as User;
    }
    return users;
  }
}
