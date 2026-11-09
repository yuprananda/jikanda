# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
# Reuse the maintainer-supplied mascot mask. Never trace or redraw its geometry.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$root = Split-Path -Parent $PSScriptRoot
$output = Join-Path $root 'assets/brand/exports'
$mask = [Drawing.Bitmap]::new((Join-Path $output 'mascot-concept-mask.png'))
$sizes = @(16,24,32,48,64,128,256)
$frames = @()
try {
    foreach ($size in $sizes) {
        $bitmap = [Drawing.Bitmap]::new($size, $size, [Drawing.Imaging.PixelFormat]::Format32bppArgb)
        $graphics = [Drawing.Graphics]::FromImage($bitmap)
        $path = [Drawing.Drawing2D.GraphicsPath]::new()
        try {
            $graphics.Clear([Drawing.Color]::Transparent)
            $graphics.SmoothingMode = [Drawing.Drawing2D.SmoothingMode]::AntiAlias
            $graphics.InterpolationMode = [Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
            $radius = [single]($size * 0.22)
            $edge = [single]($size - 1)
            $path.AddArc(0,0,$radius,$radius,180,90)
            $path.AddArc(($edge-$radius),0,$radius,$radius,270,90)
            $path.AddArc(($edge-$radius),($edge-$radius),$radius,$radius,0,90)
            $path.AddArc(0,($edge-$radius),$radius,$radius,90,90)
            $path.CloseFigure()
            $brush = [Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(255,24,24,24))
            try { $graphics.FillPath($brush,$path) } finally { $brush.Dispose() }
            $height = [single]($size * 0.84)
            $width = [single]($height * $mask.Width / $mask.Height)
            $graphics.DrawImage($mask, [Drawing.RectangleF]::new(($size-$width)/2,($size-$height)/2,$width,$height))
            $filename = Join-Path $output "jikanda-$size.png"
            $bitmap.Save($filename,[Drawing.Imaging.ImageFormat]::Png)
            $frames += ,([IO.File]::ReadAllBytes($filename))
        } finally { $path.Dispose(); $graphics.Dispose(); $bitmap.Dispose() }
    }
} finally { $mask.Dispose() }
$stream = [IO.File]::Create((Join-Path $output 'jikanda.ico'))
$writer = [IO.BinaryWriter]::new($stream)
try {
    $writer.Write([uint16]0); $writer.Write([uint16]1); $writer.Write([uint16]$sizes.Count)
    $offset = 6 + 16 * $sizes.Count
    for ($i=0; $i -lt $sizes.Count; $i++) {
        $dimension = if ($sizes[$i] -eq 256) { 0 } else { $sizes[$i] }
        $writer.Write([byte]$dimension); $writer.Write([byte]$dimension)
        $writer.Write([byte]0); $writer.Write([byte]0)
        $writer.Write([uint16]1); $writer.Write([uint16]32)
        $writer.Write([uint32]$frames[$i].Length); $writer.Write([uint32]$offset)
        $offset += $frames[$i].Length
    }
    foreach ($frame in $frames) { $writer.Write([byte[]]$frame) }
} finally { $writer.Dispose() }
Write-Output 'Exported original mascot app icons: 16, 24, 32, 48, 64, 128, 256 px and multi-resolution ICO.'
