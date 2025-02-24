import { CommonModule } from '@angular/common';
import { Component, signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { UploadResponse } from '../http/http.models';
import { FileUploaderComponent } from "./file-uploader/file-uploader.component";
import { UploadInformationComponent } from './upload-information/upload-information.component';

@Component({
  selector: 'tc-upload',
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatChipsModule,
    FileUploaderComponent,
    UploadInformationComponent,
  ],
  templateUrl: './upload.component.html',
  styleUrl: './upload.component.scss'
})
export class UploadComponent {
  protected readonly encryptedFile = signal<UploadResponse | null>(null);

  protected onEncryptedFile(encryptedFile: UploadResponse) {
    this.encryptedFile.set(encryptedFile);
  }
}
