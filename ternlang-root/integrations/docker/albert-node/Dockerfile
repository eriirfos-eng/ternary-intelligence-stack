# (C) 2026 RFI-IRFOS Graz Institute (ZVR: 1015608684)
# Licensed under the Business Source License 1.1 (BSL-1.1).
# Patent Reference: A50296/2026.
# For licensing inquiries, contact: licensing@ternlang.com

FROM alpine:latest

LABEL org.opencontainers.image.authors="RFI-IRFOS Graz Institute" \
      org.opencontainers.image.vendor="RFI-IRFOS" \
      org.opencontainers.image.title="albert-node" \
      org.opencontainers.image.description="Ternlang Compiler and MoE-13 Orchestrator Node" \
      at.rfi-irfos.patent="A50296/2026" \
      at.rfi-irfos.license="BSL-1.1"

# Install dependencies
RUN apk add --no-cache curl build-base python3 py3-pip git

# Mock Installation of ternlang-compiler and MoE-13
# In production, this would clone and build from the private TIS repositories.
RUN mkdir -p /opt/tis/bin && \
    echo '#!/bin/sh\necho "Ternlang Compiler (RFI-IRFOS) v0.1.0"' > /opt/tis/bin/ternc && \
    echo '#!/bin/sh\necho "MoE-13 Orchestrator (RFI-IRFOS) v1.0.0"' > /opt/tis/bin/moe13 && \
    chmod +x /opt/tis/bin/ternc /opt/tis/bin/moe13

ENV PATH="/opt/tis/bin:${PATH}"

WORKDIR /workspace

ENTRYPOINT ["moe13"]
CMD ["--help"]
