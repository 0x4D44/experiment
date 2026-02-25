export interface Rectangle { x: number; y: number; width: number; height: number }
export interface Circle { x: number; y: number; radius: number }
export interface Vector2 { x: number; y: number }
export function rectColliding(rect1: Rectangle, rect2: Rectangle): boolean {
  return rect1.x < rect2.x + rect2.width && rect1.x + rect1.width > rect2.x && rect1.y < rect2.y + rect2.height && rect1.y + rect1.height > rect2.y;
}
export function circleRectColliding(circle: Circle, rect: Rectangle): boolean {
  const closestX = Math.max(rect.x, Math.min(circle.x, rect.x + rect.width));
  const closestY = Math.max(rect.y, Math.min(circle.y, rect.y + rect.height));
  const distanceX = circle.x - closestX;
  const distanceY = circle.y - closestY;
  return distanceX * distanceX + distanceY * distanceY < circle.radius * circle.radius;
}
export function magnitude(v: Vector2): number { return Math.sqrt(v.x * v.x + v.y * v.y); }
export function normalizeVector(v: Vector2): Vector2 {
  const length = Math.sqrt(v.x * v.x + v.y * v.y);
  if (length === 0) return { x: 0, y: 0 };
  return { x: v.x / length, y: v.y / length };
}
export function getCollisionSide(circle: Circle, rect: Rectangle): 'top' | 'bottom' | 'left' | 'right' | 'corner' {
  const closestX = Math.max(rect.x, Math.min(circle.x, rect.x + rect.width));
  const closestY = Math.max(rect.y, Math.min(circle.y, rect.y + rect.height));
  const distanceX = circle.x - closestX;
  const distanceY = circle.y - closestY;
  const absDistX = Math.abs(distanceX);
  const absDistY = Math.abs(distanceY);
  if (absDistX > absDistY) return distanceX > 0 ? 'right' : 'left';
  return distanceY > 0 ? 'bottom' : 'top';
}
