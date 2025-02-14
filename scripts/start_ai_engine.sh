#!/bin/bash

# Start federated learning server
python -m ai-engine.ml-models.gpt6.9.training_loop \
    --num_workers 42 \
    --batch_size 690 \
    --use_gpu
