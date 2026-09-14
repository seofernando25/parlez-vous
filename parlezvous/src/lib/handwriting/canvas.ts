export type Point = { x: number; y: number };

export function drawingCoordinates(canvas: HTMLCanvasElement, event: MouseEvent | TouchEvent): Point {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const client = event instanceof MouseEvent ? event : event.touches[0];
    return { x: (client.clientX - rect.left) * scaleX, y: (client.clientY - rect.top) * scaleY };
}

export function preprocessStrokes(strokes: Point[][]): number[] {
    if (strokes.length === 0) return new Array(28 * 28).fill(0);
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    for (const stroke of strokes) for (const point of stroke) {
        minX = Math.min(minX, point.x); maxX = Math.max(maxX, point.x);
        minY = Math.min(minY, point.y); maxY = Math.max(maxY, point.y);
    }

    const width = maxX - minX;
    const height = maxY - minY;
    const scale = Math.max(width, height) > 0 ? 20 / Math.max(width, height) : 1;
    const dx = (28 - width * scale) / 2;
    const dy = (28 - height * scale) / 2;
    const canvas = document.createElement('canvas');
    canvas.width = canvas.height = 28;
    const ctx = canvas.getContext('2d')!;
    ctx.fillStyle = '#000'; ctx.fillRect(0, 0, 28, 28);
    ctx.strokeStyle = '#fff'; ctx.lineWidth = 3.5; ctx.lineCap = 'round'; ctx.lineJoin = 'round';

    for (const stroke of strokes) {
        if (!stroke.length) continue;
        ctx.beginPath();
        ctx.moveTo((stroke[0].x - minX) * scale + dx, (stroke[0].y - minY) * scale + dy);
        for (const point of stroke.slice(1)) ctx.lineTo((point.x - minX) * scale + dx, (point.y - minY) * scale + dy);
        ctx.stroke();
    }
    const data = ctx.getImageData(0, 0, 28, 28).data;
    return Array.from({ length: 28 * 28 }, (_, i) => data[i * 4] / 255 > 0.3 ? 255 : 0);
}
