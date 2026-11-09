# Provisional raster extraction only. This does not create or approve vector masters.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$projectRoot = Split-Path -Parent $PSScriptRoot
$outputDirectory = Join-Path $projectRoot 'assets/brand/exports'
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
$source = [Drawing.Bitmap]::new((Join-Path $projectRoot 'Jikanda Brand Identity Sheet.png'))
try {
    $regions = @(
        @{ Name = 'mascot-concept-mask.png'; X = 635; Y = 50; Width = 315; Height = 335 },
        @{ Name = 'wordmark-concept-mask.png'; X = 1035; Y = 159; Width = 450; Height = 145 }
    )
    foreach ($region in $regions) {
        $mask = [Drawing.Bitmap]::new($region.Width, $region.Height, [Drawing.Imaging.PixelFormat]::Format32bppArgb)
        try {
            for ($y = 0; $y -lt $region.Height; $y++) {
                for ($x = 0; $x -lt $region.Width; $x++) {
                    $pixel = $source.GetPixel(($region.X + $x), ($region.Y + $y))
                    # The sheet has purple artwork on near-white. Preserve silhouette
                    # and antialiasing as alpha; Slint supplies the monochrome color.
                    $minimum = [Math]::Min($pixel.R, [Math]::Min($pixel.G, $pixel.B))
                    $alpha = [int][Math]::Round(255 * [Math]::Max(0.0, [Math]::Min(1.0, (255.0 - $minimum - 12) / 190.0)))
                    $mask.SetPixel($x, $y, [Drawing.Color]::FromArgb($alpha, 255, 255, 255))
                }
            }
            $mask.Save((Join-Path $outputDirectory $region.Name), [Drawing.Imaging.ImageFormat]::Png)
        } finally { $mask.Dispose() }
    }
} finally { $source.Dispose() }
