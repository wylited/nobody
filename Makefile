# Set the executable name
EXE = nobody

# Set the source files
IMGUI_DIR = imgui
SOURCES = src/main.cpp
SOURCES += $(IMGUI_DIR)/imgui.cpp $(IMGUI_DIR)/imgui_demo.cpp $(IMGUI_DIR)/imgui_draw.cpp $(IMGUI_DIR)/imgui_tables.cpp $(IMGUI_DIR)/imgui_widgets.cpp
SOURCES += $(IMGUI_DIR)/backends/imgui_impl_glfw.cpp $(IMGUI_DIR)/backends/imgui_impl_opengl3.cpp

# Set the object files
OBJECTS = $(patsubst %.cpp, %.o, $(SOURCES))

# Set the include directories
INCLUDES = -Iimgui -Iimgui/backends

# Set the compiler flags
CXXFLAGS = -std=c++11 -Wall -Wformat -g
CXXFLAGS += -lGL

# Set the linker flags
LINUX_GL_LIBS = -lGL
ifeq ($(UNAME_S), Linux) #LINUX
	ECHO_MESSAGE = "Linux"
	LINUX_GL_LIBS = -lGL
	LIBS += $(LINUX_GL_LIBS)
	LIBS += `pkg-config --static --libs glfw3`

	CXXFLAGS += `pkg-config --cflags glfw3`
	CFLAGS = $(CXXFLAGS)
endif

ifeq ($(UNAME_S), Darwin) #APPLE
	ECHO_MESSAGE = "Mac OS X"
	LIBS += -framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo
	LIBS += -L/usr/local/lib -L/opt/local/lib -L/opt/homebrew/lib
	#LIBS += -lglfw3
	LIBS += -lglfw

	CXXFLAGS += -I/usr/local/include -I/opt/local/include -I/opt/homebrew/include
	CFLAGS = $(CXXFLAGS)
endif

ifeq ($(OS), Windows_NT)
	ECHO_MESSAGE = "MinGW"
	LIBS += -lglfw3 -lgdi32 -lopengl32 -limm32

	CXXFLAGS += `pkg-config --cflags glfw3`
	CFLAGS = $(CXXFLAGS)
endif

# Set the platform-specific flags
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Linux)
	LDFLAGS += `pkg-config --static --libs glfw3`
endif
ifeq ($(UNAME_S), Darwin)
	LDFLAGS += -framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo
endif

# Set the build rules
all: $(EXE)

$(EXE): $(OBJECTS)
	$(CXX) $(LDFLAGS) $(LIBS) -lGL -o $@ $^

%.o: %.cpp
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c -o $@ $<

clean:
	rm -f $(EXE) $(OBJECTS)

.PHONY: all clean
