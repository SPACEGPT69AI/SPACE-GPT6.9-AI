import locust
from spacex_adapter import SpaceXDataPipeline

class SpaceLoadTest(locust.HttpUser):
    @task(5)
    def test_starlink_api(self):
        self.client.get("/api/starlink")
    
    @task(1)
    def test_heavy_compute(self):
        pipeline = SpaceXDataPipeline("TEST_KEY")
        data = pipeline.get_starlink_data()
        self.client.post("/ai/process", json=data)
