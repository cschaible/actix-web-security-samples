import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {UserResource} from '../resources/user/user-resource';
import {environment} from '../../environments/environment';
import {SaveUserResource} from '../resources/user/save-user-resource';
import {RegisterUserResource} from '../resources/user/register-user-resource';
import {ListResource} from '../resources/list-resource';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(private httpClient: HttpClient) {
  }

  public getCurrentUser(): Observable<UserResource> {
    return this.httpClient.get<UserResource>(environment.api + '/users/current');
  }

  public registerCurrentUser(user: RegisterUserResource): Observable<UserResource> {
    return this.httpClient.post<UserResource>(environment.api + '/users/current', user);
  }

  public updateCurrentUser(user: SaveUserResource): Observable<UserResource> {
    return this.httpClient.put<UserResource>(environment.api + '/users/current', user);
  }

  // tslint:disable-next-line:ban-types
  public deleteCurrentUser(): Observable<Object> {
    return this.httpClient.delete(environment.api + '/users/current');
  }

  public findAllUsers(): Observable<ListResource<UserResource>> {
    return this.httpClient.get<ListResource<UserResource>>(environment.api + '/users');
  }

  public updateUser(user: SaveUserResource, identifier: string): Observable<UserResource> {
    return this.httpClient.put<UserResource>(environment.api + `/users/${identifier}`, user);
  }

  // tslint:disable-next-line:ban-types
  public deleteUser(identifier: string): Observable<Object> {
    return this.httpClient.delete(environment.api + `/users/${identifier}`);
  }
}
