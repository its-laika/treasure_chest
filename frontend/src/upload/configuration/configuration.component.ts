import { Component, input, signal } from '@angular/core';
import { Configuration } from '../../http/http.models';
import { ToReadableBytesPipe } from '../../conversion/to-readable-bytes.pipe';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
@Component({
  selector: 'tc-configuration',
  imports: [ToReadableBytesPipe, MatChipsModule, MatIconModule],
  templateUrl: './configuration.component.html',
  styleUrl: './configuration.component.scss'
})
export class ConfigurationComponent {
  public readonly configuration = input.required<Configuration>();
}
