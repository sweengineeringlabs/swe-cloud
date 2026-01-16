import urllib.request
import urllib.error
import time
import sys
import json

AZURE_HOST = "http://127.0.0.1:4566"
GCP_HOST = "http://127.0.0.1:4567"

def request(method, url, data=None):
    req = urllib.request.Request(url, method=method)
    if data:
        req.add_header('Content-Type', 'application/json')
        req.data = json.dumps(data).encode('utf-8')
    
    try:
        with urllib.request.urlopen(req) as response:
            return {
                "status": response.status,
                "body": response.read().decode('utf-8'),
                "headers": dict(response.getheaders())
            }
    except urllib.error.HTTPError as e:
        return {
            "status": e.code,
            "body": e.read().decode('utf-8'),
            "error": True
        }
    except Exception as e:
        return {"error": str(e), "status": 0}

def wait_for_port(port, retries=20):
    url = f"http://127.0.0.1:{port}/health"
    for i in range(retries):
        try:
            with urllib.request.urlopen(url, timeout=1) as response:
                if response.status == 200:
                    print(f"Port {port} is ready.")
                    return True
        except:
            print(f"Waiting for port {port}...")
            time.sleep(1)
    return False

def test_azure():
    print("\n=== Testing Azure Data API ===")
    ts = int(time.time())
    
    # Blob
    acc = f"testacc{ts}"
    cont = f"testcont{ts}"
    blob = f"testblob{ts}"
    print(f"Creating container {cont}...")
    res = request("PUT", f"{AZURE_HOST}/blob/{acc}/{cont}", {"public_access": "container"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Putting blob {blob}...")
    res = request("PUT", f"{AZURE_HOST}/blob/{acc}/{cont}/{blob}", {"content": "hello_azure", "content_type": "text/plain"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting blob {blob}...")
    res = request("GET", f"{AZURE_HOST}/blob/{acc}/{cont}/{blob}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200 or res.get('body') != "hello_azure": return False

    # Cosmos
    db = f"testdb{ts}"
    coll = f"testcoll{ts}"
    print(f"Creating Cosmos DB {db}...")
    res = request("PUT", f"{AZURE_HOST}/cosmos/{acc}/{db}", {"throughput": 400})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Creating Collection {coll}...")
    res = request("PUT", f"{AZURE_HOST}/cosmos/{acc}/{db}/{coll}", {"partition_key_path": "/pk"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Creating Item...")
    item_id = "item1"
    pk = "pkval"
    res = request("POST", f"{AZURE_HOST}/cosmos/{acc}/{db}/{coll}/items", {
        "item": {"id": item_id, "pk": pk, "value": "data"},
        "partition_key": pk 
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting Item...")
    res = request("GET", f"{AZURE_HOST}/cosmos/{acc}/{db}/{coll}/items/{item_id}/{pk}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False
    
    # Compute (VM)
    rg = f"testrg{ts}"
    vm = f"testvm{ts}"
    print(f"Creating VM {vm}...")
    res = request("PUT", f"{AZURE_HOST}/compute/{rg}/vms/{vm}", {
        "location": "westus",
        "vm_size": "Standard_B1s",
        "os_type": "Linux",
        "admin_username": "azureuser"
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting VM {vm}...")
    res = request("GET", f"{AZURE_HOST}/compute/{rg}/vms/{vm}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    # Event Grid
    topic = f"testtopic{ts}"
    print(f"Creating EventGrid Topic {topic}...")
    res = request("PUT", f"{AZURE_HOST}/eventgrid/{rg}/topics/{topic}", {
        "location": "westus"
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    return True

def test_gcp():
    print("\n=== Testing GCP Data API ===")
    ts = int(time.time())

    # GCS
    bucket = f"testbucket{ts}"
    obj = f"testobj{ts}"
    print(f"Creating Bucket {bucket}...")
    res = request("PUT", f"{GCP_HOST}/storage/{bucket}", {"location": "us-east1"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Putting Object {obj}...")
    res = request("PUT", f"{GCP_HOST}/storage/{bucket}/o/{obj}", {"content": "hello_gcp", "content_type": "text/plain"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting Object {obj}...")
    res = request("GET", f"{GCP_HOST}/storage/{bucket}/o/{obj}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200 or res.get('body') != "hello_gcp": return False

    # Firestore
    proj = f"testproj{ts}"
    db = f"testdb-fs{ts}"
    coll = "users"
    doc = "user1"
    
    print(f"Creating Firestore DB {db}...")
    res = request("PUT", f"{GCP_HOST}/firestore/{proj}/databases/{db}", {"location_id": "us", "database_type": "native"})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Creating Document {doc}...")
    res = request("PUT", f"{GCP_HOST}/firestore/{proj}/databases/{db}/documents/{coll}/{doc}", {
        "fields": {"name": "Alice", "age": 30}
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting Document {doc}...")
    res = request("GET", f"{GCP_HOST}/firestore/{proj}/databases/{db}/documents/{coll}/{doc}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    # Compute (Instances)
    zone = "us-central1-a"
    inst = f"testinst{ts}"
    print(f"Creating Instance {inst}...")
    res = request("PUT", f"{GCP_HOST}/compute/{proj}/zones/{zone}/instances/{inst}", {
        "machine_type": "e2-medium",
        "image": "debian-11",
        "network": "default"
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Getting Instance {inst}...")
    res = request("GET", f"{GCP_HOST}/compute/{proj}/zones/{zone}/instances/{inst}")
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    # PubSub
    topic = f"testtopic{ts}"
    sub = f"testsub{ts}"
    print(f"Creating PubSub Topic {topic}...")
    res = request("PUT", f"{GCP_HOST}/pubsub/{proj}/topics/{topic}", {})
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False

    print(f"Creating PubSub Subscription {sub}...")
    res = request("PUT", f"{GCP_HOST}/pubsub/{proj}/subscriptions/{sub}", {
        "topic": topic,
        "push_endpoint": "http://localhost/push"
    })
    print(f"Status: {res['status']}, Body: {res.get('body')}")
    if res['status'] != 200: return False
    
    return True

if __name__ == "__main__":
    if not wait_for_port(4566):
        print("Azure API not ready.")
        sys.exit(1)
    if not wait_for_port(4567):
        print("GCP API not ready.")
        sys.exit(1)
    
    if test_azure() and test_gcp():
        print("\nAll tests passed!")
        sys.exit(0)
    else:
        print("\nSome tests failed.")
        sys.exit(1)
