
PROTOC := protoc
PROTO_DIR := protobuf_definitions
PROTOS := $(wildcard $(PROTO_DIR)/*.proto)

PROTO_HEADERS := $(PROTOS:%.proto=%.pb.h)
PROTO_CFILES := $(PROTOS:%.proto=%.pb.cc)

PROTO_OUTPUTS := $(PROTO_CFILES) $(PROTO_HEADERS)

OBJECTS := $(PROTO_CFILES:%.cc=%.o)

CXXFLAGS += -O2 -Wall -std=c++17 -fPIC
LDFLAGS += -fPIC -lprotobuf

ifeq ($(PREFIX),)
        PREFIX=/usr
endif

SO := libblueyeprotocol.so

$(SO): $(OBJECTS)
	$(CXX) -shared $(LDFLAGS) -o $@ $^

$(PROTO_DIR)/%.o: $(PROTO_DIR)/%.cc $(PROTO_OUTPUTS)
	$(CXX) $(CXXFLAGS) -o $@ -c $<

$(PROTO_OUTPUTS): $(PROTOS)
	$(PROTOC) -I$(PROTO_DIR) --cpp_out=$(PROTO_DIR) $^

clean:
	@rm -f $(PROTO_OUTPUTS) $(OBJECTS) $(SO)

install:
	@install -d $(DESTDIR)$(PREFIX)/lib
	@install -m 755 $(SO) $(DESTDIR)$(PREFIX)/lib
	@install -d $(DESTDIR)$(PREFIX)/include/blueyeprotocol
	@install -m 755 $(PROTO_HEADERS) $(DESTDIR)$(PREFIX)/include/blueyeprotocol

uninstall:
	@rm $(DESTDIR)$(PREFIX)/lib/$(SO)
	@rm -r $(DESTDIR)$(PREFIX)/include/blueyeprotocol
