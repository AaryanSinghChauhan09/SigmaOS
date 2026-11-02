"""SigmaOS-draw: Vector drawing and screen annotation application.

Provides SVG-based drawing, shape primitives, and presentation annotation
tools integrated with the SigmaOS desktop environment.
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass, field
from typing import List, Optional


# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------

@dataclass
class Point:
    x: float
    y: float

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"


@dataclass
class Shape:
    """Base class for all drawable shapes."""
    stroke: str = "#000000"
    fill: str = "none"
    stroke_width: float = 1.0

    def to_svg(self) -> str:
        raise NotImplementedError


@dataclass
class Rectangle(Shape):
    origin: Point = field(default_factory=lambda: Point(0, 0))
    width: float = 100.0
    height: float = 60.0

    def to_svg(self) -> str:
        return (
            f'<rect x="{self.origin.x}" y="{self.origin.y}" '
            f'width="{self.width}" height="{self.height}" '
            f'stroke="{self.stroke}" fill="{self.fill}" '
            f'stroke-width="{self.stroke_width}" />'
        )


@dataclass
class Ellipse(Shape):
    center: Point = field(default_factory=lambda: Point(50, 50))
    rx: float = 50.0
    ry: float = 30.0

    def to_svg(self) -> str:
        return (
            f'<ellipse cx="{self.center.x}" cy="{self.center.y}" '
            f'rx="{self.rx}" ry="{self.ry}" '
            f'stroke="{self.stroke}" fill="{self.fill}" '
            f'stroke-width="{self.stroke_width}" />'
        )


@dataclass
class Arrow(Shape):
    start: Point = field(default_factory=lambda: Point(0, 0))
    end: Point = field(default_factory=lambda: Point(100, 100))

    def to_svg(self) -> str:
        return (
            f'<line x1="{self.start.x}" y1="{self.start.y}" '
            f'x2="{self.end.x}" y2="{self.end.y}" '
            f'stroke="{self.stroke}" stroke-width="{self.stroke_width}" '
            f'marker-end="url(#arrowhead)" />'
        )


# ---------------------------------------------------------------------------
# Canvas / document
# ---------------------------------------------------------------------------

@dataclass
class Layer:
    name: str
    shapes: List[Shape] = field(default_factory=list)
    opacity: float = 1.0

    def add_shape(self, shape: Shape) -> None:
        self.shapes.append(shape)

    def to_svg_group(self) -> str:
        inner = "\n    ".join(s.to_svg() for s in self.shapes)
        return (
            f'<g id="{self.name}" opacity="{self.opacity}">\n    {inner}\n  </g>'
        )


class Canvas:
    """Represents an SVG drawing canvas with multiple layers."""

    ARROWHEAD_DEF = (
        '<defs><marker id="arrowhead" markerWidth="10" markerHeight="7" '
        'refX="10" refY="3.5" orient="auto">'
        '<polygon points="0 0, 10 3.5, 0 7" fill="#000" /></marker></defs>'
    )

    def __init__(self, width: int = 800, height: int = 600) -> None:
        self.width = width
        self.height = height
        self.layers: List[Layer] = []

    def add_layer(self, name: str, opacity: float = 1.0) -> Layer:
        layer = Layer(name=name, opacity=opacity)
        self.layers.append(layer)
        return layer

    def to_svg(self) -> str:
        groups = "\n  ".join(layer.to_svg_group() for layer in self.layers)
        return (
            f'<?xml version="1.0" encoding="UTF-8"?>\n'
            f'<svg xmlns="http://www.w3.org/2000/svg" '
            f'width="{self.width}" height="{self.height}">\n'
            f'  {self.ARROWHEAD_DEF}\n'
            f'  {groups}\n'
            f'</svg>'
        )

    def save(self, path: str) -> None:
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(self.to_svg())
        print(f"[draw] Saved SVG to {path}")


# ---------------------------------------------------------------------------
# Annotation overlay
# ---------------------------------------------------------------------------

class AnnotationSession:
    """Manages a screen annotation overlay session."""

    def __init__(self, canvas: Canvas) -> None:
        self._canvas = canvas
        self._active = False

    def start(self) -> None:
        self._active = True
        print("[draw] Annotation session started.")

    def stop(self) -> None:
        self._active = False
        print("[draw] Annotation session stopped.")

    @property
    def is_active(self) -> bool:
        return self._active

    def add_highlight(self, x: float, y: float, width: float, height: float,
                      color: str = "#FFFF0066") -> None:
        if not self._active:
            raise RuntimeError("Annotation session is not active.")
        layer = self._canvas.layers[0] if self._canvas.layers else self._canvas.add_layer("annotations")
        layer.add_shape(Rectangle(fill=color, stroke="none", origin=Point(x, y),
                                  width=width, height=height))


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def build_demo_canvas() -> Canvas:
    canvas = Canvas(width=800, height=600)
    layer = canvas.add_layer("default")
    layer.add_shape(Rectangle(origin=Point(10, 10), width=200, height=100,
                               stroke="#3355aa", stroke_width=2.0))
    layer.add_shape(Ellipse(center=Point(400, 300), rx=120, ry=80,
                            stroke="#aa3355", fill="#ffeedd"))
    layer.add_shape(Arrow(start=Point(50, 500), end=Point(750, 500),
                          stroke="#005500", stroke_width=3.0))
    return canvas


def parse_args(argv: Optional[List[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="sigmadraw",
        description="SigmaOS vector drawing and annotation tool.",
    )
    parser.add_argument(
        "--output", "-o", default="output.svg",
        help="Output SVG file path (default: output.svg)",
    )
    parser.add_argument(
        "--demo", action="store_true",
        help="Render a built-in demo drawing and exit",
    )
    return parser.parse_args(argv)


def main(argv: Optional[List[str]] = None) -> int:
    args = parse_args(argv)
    if args.demo:
        canvas = build_demo_canvas()
        canvas.save(args.output)
        print("[draw] Demo SVG written successfully.")
        return 0

    print("[draw] SigmaOS-draw is running in headless mode.")
    print("[draw] Pass --demo to generate a sample SVG, or integrate with the GTK4 frontend.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
