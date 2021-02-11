import {BaseUser} from './base-user';

export class User extends BaseUser {
  public type = 'User';
  public id: string;
  public version: number;
  public firstName: string;
  public lastName: string;
  public admin: boolean;
}
