import { CommonModule } from '@angular/common';
import { Component, computed, ElementRef, inject, signal, viewChild } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { HttpService } from '../http/http.service';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { Configuration } from '../http/http.models';
import { ConfigurationComponent } from "./configuration/configuration.component";
import { FileInfoComponent } from './file-info/file-info.component';
import { FileChooserComponent } from './file-chooser/file-chooser.component';

@Component({
  selector: 'tc-upload',
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatChipsModule,
    ConfigurationComponent,
    FileInfoComponent,
    FileChooserComponent,
  ],
  templateUrl: './upload.component.html',
  styleUrl: './upload.component.scss'
})
export class UploadComponent {
  private readonly httpService = inject(HttpService);

  protected readonly config = signal<Configuration | null>(null);
  protected readonly file = signal<File | null>(null);
  protected readonly uploadAllowed = computed(() => {
    const configuration = this.config();
    const file = this.file();

    return !!configuration && !!file && file.size <= configuration.BodyMaxSize;
  });

  constructor() {
    this.httpService.loadOptions()
      .pipe(takeUntilDestroyed())
      .subscribe(this.config.set);
  }


  protected onFile(file: File) {
    this.file.set(file);
  }

  protected onUploadClick() {
    const file = this.file();
    if (!file) {
      return;
    }

    this.httpService.uploadFile(file).subscribe(result => console.log(result));
  }
}
