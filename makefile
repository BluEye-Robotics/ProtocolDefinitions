
PROTOC := protoc
PROTO_DIR := protobuf_definitions
PROTOS := $(wildcard $(PROTO_DIR)/*.proto)

PROTO_HEADERS = $(PROTOS:%.proto=%.pb.h)
PROTO_CFILES = $(PROTOS:%.proto=%.pb.cc)

PROTO_OUTPUTS = $(PROTO_CFILES) $(PROTO_HEADERS)


CXXFLAGS := -O2 -Wall -std=c++17 -fPIC
LDFLAGS := -fPIC

OBJECTS = $(PROTO_CFILES:%.cc=%.o)

SO := libprotocoldefinitions.so

$(SO): $(OBJECTS)
	$(CXX) -shared $(LDFLAGS) -o $@ $^

$(PROTO_DIR)/%.o: $(PROTO_DIR)/%.cc
	$(CXX) $(CXXFLAGS) -o $@ -c $<

$(PROTO_OUTPUTS): $(PROTOS)
	$(PROTOC) -I$(PROTO_DIR) --cpp_out=$(PROTO_DIR) $^

clean:
	@rm -f $(PROTO_OUTPUTS) $(OBJECTS) $(SO)

install:
	@install -d $(DESTDIR)$(PREFIX)/lib
	@install -m 755 $(SO) $(DESTDIR)$(PREFIX)/lib

uninstall:
	@rm $(DESTDIR)$(PREFIX)/lib/$(SO)
