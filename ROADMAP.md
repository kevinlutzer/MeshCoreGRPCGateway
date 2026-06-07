# MeshCoreGRPCGateway Roadmap

This document outlines the current state and planned features for the MeshCoreGRPCGateway project.

## Current State

### ✅ Completed Features

- **CLI Tool** - Full-featured command-line interface for interacting with the MeshCore system
- **gRPC Service** - Robust gRPC server implementation for remote control and management
- **Message Transport** - Ability to send and receive messages through private devices and channels
- **Snapcraft Integration** - Working `snapcraft.yaml` configuration for building and distributing the snap
- **Snap Packaging** - Complete support for creating installable snap packages

## Upcoming Features

### Remote DFU (Device Firmware Update)

Implement remote Device Firmware Update capabilities to allow over-the-air firmware updates for MeshCore devices without requiring physical access to the hardware.

**Goals:**
- Enable remote firmware updates through the gRPC gateway
- Support for multiple devices simultaneously
- Progress tracking and status reporting
- Rollback capabilities for failed updates
- Verification of firmware integrity post-update

**Expected Impact:**
- Simplified device maintenance and management
- Reduced downtime for firmware updates
- Improved security through timely security patches

### GPIO Reset Support

Add support for hardware reset functionality through GPIO (General Purpose Input/Output) control on MeshHat devices.

**Goals:**
- Implement GPIO pin control through the gRPC API
- Support for hardware reset sequences
- Safe reset procedures with validation
- Configuration options for different GPIO pin layouts
- Status monitoring for reset operations

**Expected Impact:**
- Enhanced device control capabilities
- Support for hardware troubleshooting
- Better integration with MeshHat hardware

## Planned Enhancements

### Phase 1: Core Functionality Expansion
- [ ] Remote DFU implementation
- [ ] GPIO Reset support
- [ ] Enhanced error handling and diagnostics
- [ ] Improved logging and debugging capabilities

### Phase 2: Reliability & Performance
- [ ] Performance optimization and benchmarking
- [ ] Connection pooling and resource management
- [ ] Graceful shutdown and recovery mechanisms
- [ ] Rate limiting and request queuing

### Phase 3: Security
- [ ] Authentication mechanisms
- [ ] Authorization and access control
- [ ] TLS/SSL support for secure communication
- [ ] API key management

### Phase 4: Observability
- [ ] Structured logging
- [ ] Metrics collection (Prometheus compatible)
- [ ] Health check endpoints
- [ ] Distributed tracing support

### Phase 5: Documentation & DevEx
- [ ] API documentation generation
- [ ] Developer guides and examples
- [ ] Architecture documentation
- [ ] Troubleshooting guides

## Ongoing Maintenance

- Dependency updates and security patches
- Bug fixes and performance improvements
- Community feedback and issue resolution
- Test coverage improvements
- Documentation updates

## Contributing

We welcome community contributions! If you're interested in working on any of these features, please open an issue or submit a pull request.

---

Last Updated: June 2026
