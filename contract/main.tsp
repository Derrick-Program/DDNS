import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi";
import "@typespec/openapi3";

using Http;
using Rest;
using TypeSpec.OpenAPI;

@service(#{ title: "DDNS Service" })
@info(#{ version: "0.1.0" })
@doc("This is the API specification for the Dynamic DNS Service (DDNS). It allows users to authenticate, manage DNS records, and update their IP addresses dynamically.")
@server("https://ddns.duacodie.com", "Production")
@server("http://127.0.0.1:8080", "Local Development")
namespace Ddns;

enum IpFamily {
  v4,
  v6,
}

model ErrorResponse {
  code: string;
  message: string;
  requestId?: string;
}

model TokenPair {
  accessToken: string;
  refreshToken: string;
  tokenType: "Bearer";
  expiresInSec: int32;
}

model LoginRequest {
  username: string;
  password: string;
  deviceName?: string;
}

model RefreshRequest {
  refreshToken: string;
}

model LogoutRequest {
  refreshToken: string;
}

model RecordInfo {
  recordId: string;
  zoneId: string;
  name: string;
  type: "A" | "AAAA";
  proxied?: boolean;
  ttl?: int32;
}

model ListRecordsResponse {
  records: RecordInfo[];
}

model UpdateRecordRequest {
  recordId: string;
  ip: string;
  family: IpFamily;
  detectedBy?: "ifconfig" | "stun" | "http" | "cloudflare-trace";
}

model UpdateRecordResponse {
  recordId: string;
  name: string;
  type: "A" | "AAAA";
  oldContent?: string;
  newContent: string;
  updatedAt: utcDateTime;
}

interface Auth {
  @route("/auth/login")
  @post
  login(@body body: LoginRequest): TokenPair | ErrorResponse;

  @route("/auth/refresh")
  @post
  refresh(@body body: RefreshRequest): TokenPair | ErrorResponse;

  @route("/auth/logout")
  @post
  logout(@body body: LogoutRequest): {} | ErrorResponse;
}

@useAuth(BearerAuth)
interface Records {
  @route("/records")
  @get
  list(): ListRecordsResponse | ErrorResponse;
}

@useAuth(BearerAuth)
interface Updates {
  @route("/records:update")
  @post
  update(@body body: UpdateRecordRequest): UpdateRecordResponse | ErrorResponse;
}
