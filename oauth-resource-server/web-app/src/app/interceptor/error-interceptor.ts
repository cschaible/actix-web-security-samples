import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {MatSnackBar} from '@angular/material/snack-bar';
import {Injectable, NgZone} from '@angular/core';
import {Observable, of} from 'rxjs';
import {catchError} from 'rxjs/operators';

@Injectable()
export class ErrorInterceptor implements HttpInterceptor {
  constructor(private snackBar: MatSnackBar,
              private readonly zone: NgZone) {
  }

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(req).pipe(
      catchError(error => this.handleError(error))
    );
  }

  handleError(error): Observable<HttpEvent<any>> {
    this.zone.run(() => {
      let errorMessage: string;
      if (error.error !== undefined) {
        errorMessage = error.error;
      } else {
        errorMessage = error.message;
      }
      this.snackBar.open(errorMessage, undefined, {
        duration: 3000,
        horizontalPosition: 'right',
        verticalPosition: 'bottom',
      });
    });
    return of(error);
  }
}
