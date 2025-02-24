import { CommonModule } from '@angular/common';
import { Component, computed, DestroyRef, inject, output, signal } from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { Configuration, UploadResponse } from '../../http/http.models';
import { HttpService } from '../../http/http.service';
import { ConfigurationComponent } from '../configuration/configuration.component';
import { FileChooserComponent } from '../file-chooser/file-chooser.component';
import { FileInfoComponent } from '../file-info/file-info.component';

@Component({
  selector: 'tc-file-uploader',
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
  templateUrl: './file-uploader.component.html',
  styleUrl: './file-uploader.component.scss'
})
export class FileUploaderComponent {
  public readonly encryptedFile = output<UploadResponse>();

  private readonly httpService = inject(HttpService);
  private readonly destroyRef = inject(DestroyRef)

  protected readonly config = signal<Configuration | null>(null);
  protected readonly file = signal<File | null>(null);

  protected readonly uploadAllowed = computed(() => {
    const configuration = this.config();
    const file = this.file();

    return !!configuration && !!file && file.size <= configuration.BodyMaxSize;
  });

  constructor() {
    this.httpService.loadOptions()
      // TODO: Error handling
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

    this.httpService.uploadFile(file)
      // TODO: Error handling
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe(response => this.encryptedFile.emit(response));
  }
}
