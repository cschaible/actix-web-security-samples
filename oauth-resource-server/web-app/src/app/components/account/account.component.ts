import {Component, OnInit} from '@angular/core';

@Component({
  selector: 'app-account',
  templateUrl: './account.component.html',
  styleUrls: ['./account.component.scss']
})
export class AccountComponent implements OnInit {

  title = 'User Details';

  btnSaveTitle = 'Save';

  btnDeleteTitle = 'Delete Account';

  btnDeleteEnabled = true;

  constructor() {
  }

  ngOnInit(): void {
  }

}
