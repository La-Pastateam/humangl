GLSLC="/usr/bin/glslc"
GLSLC_42="$HOME/.local/bin/glslc"

if [ -x "$GLSLC" ]; then
	$GLSLC shader.vert -o vert.spv
	$GLSLC shader.frag -o frag.spv
	$GLSLC imgui.vert -o imgui.vert.spv
	$GLSLC imgui.frag -o imgui.frag.spv
else
	$GLSLC_42 shader.vert -o vert.spv
	$GLSLC_42 shader.frag -o frag.spv
	$GLSLC_42 imgui.vert -o imgui.vert.spv
	$GLSLC_42 imgui.frag -o imgui.frag.spv
fi
