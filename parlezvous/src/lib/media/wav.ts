function writeAscii(view: DataView, offset: number, value: string) {
    for (let i = 0; i < value.length; i++) {
        view.setUint8(offset + i, value.charCodeAt(i));
    }
}

function writePcm16(view: DataView, offset: number, samples: Float32Array) {
    for (let i = 0; i < samples.length; i++, offset += 2) {
        const sample = Math.max(-1, Math.min(1, samples[i]));
        view.setInt16(offset, sample < 0 ? sample * 0x8000 : sample * 0x7fff, true);
    }
}

export function mergeFloat32Buffers(buffers: readonly Float32Array[], length: number): Float32Array {
    const merged = new Float32Array(length);
    let offset = 0;
    for (const buffer of buffers) {
        merged.set(buffer, offset);
        offset += buffer.length;
    }
    return merged;
}

export function encodeMonoPcm16Wav(samples: Float32Array, sampleRate: number): DataView {
    const bytesPerSample = 2;
    const dataSize = samples.length * bytesPerSample;
    const buffer = new ArrayBuffer(44 + dataSize);
    const view = new DataView(buffer);

    writeAscii(view, 0, 'RIFF');
    view.setUint32(4, 36 + dataSize, true);
    writeAscii(view, 8, 'WAVE');
    writeAscii(view, 12, 'fmt ');
    view.setUint32(16, 16, true);
    view.setUint16(20, 1, true); // PCM
    view.setUint16(22, 1, true); // mono
    view.setUint32(24, sampleRate, true);
    view.setUint32(28, sampleRate * bytesPerSample, true);
    view.setUint16(32, bytesPerSample, true);
    view.setUint16(34, 16, true);
    writeAscii(view, 36, 'data');
    view.setUint32(40, dataSize, true);
    writePcm16(view, 44, samples);

    return view;
}

export function createWavBlob(
    buffers: readonly Float32Array[],
    length: number,
    sampleRate: number
): Blob {
    const samples = mergeFloat32Buffers(buffers, length);
    return new Blob([encodeMonoPcm16Wav(samples, sampleRate)], { type: 'audio/wav' });
}

export async function blobToBase64(blob: Blob): Promise<string> {
    const dataUrl = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader();
        reader.onerror = () => reject(reader.error ?? new Error('Failed to read audio blob'));
        reader.onload = () => resolve(String(reader.result));
        reader.readAsDataURL(blob);
    });

    const comma = dataUrl.indexOf(',');
    return comma >= 0 ? dataUrl.slice(comma + 1) : dataUrl;
}
