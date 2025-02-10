import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'tcReadableBytes'
})
export class ToReadableBytesPipe implements PipeTransform {
  private readonly bytesMapping = {
    TB: 1_000_000_000_000,
    GB: 1_000_000_000,
    MB: 1_000_000,
    KB: 1_000,
    bytes: 1
  }

  transform(value: any, ...args: any[]) {
    if (!value) {
      return '- bytes';
    }

    if (!Number.isInteger(value)) {
      throw new Error(`Given value '${value}' is not an int.`);
    }

    let suffix: keyof typeof this.bytesMapping;
    for (suffix in this.bytesMapping) {
      const minValue = this.bytesMapping[suffix];
      if (value / minValue < 1) {
        continue;
      }

      return `${Math.round(value / minValue)} ${suffix}`;
    }

    return '0';
  }
}
