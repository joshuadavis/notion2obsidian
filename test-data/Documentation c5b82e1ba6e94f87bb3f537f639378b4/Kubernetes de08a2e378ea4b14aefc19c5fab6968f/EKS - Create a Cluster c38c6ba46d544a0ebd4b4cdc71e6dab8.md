# EKS - Create a Cluster

[Full EKS - Amazon Kubernetes Cluster In Less Than 15 minutes](https://www.youtube.com/watch?v=zKCudYazOwM)

From Cloud9

```bash
mkdir -p ~/.kube
sudo curl --silent --location -o /usr/local/bin/kubectl "https://amazon-eks.s3-us-west-2.amazonaws.com/1.11.5/2018-12-06/bin/linux/amd64/kubectl"
sudo chmod +x /usr/local/bin/kubectl

go get -u -v github.com/kubernetes-sigs/aws-iam-authenticator/cmd/aws-iam-authenticator
sudo mv ~/go/bin/aws-iam-authenticator /usr/local/bin/aws-iam-authenticator

sudo yum -y install jq
```

Verify tools

```bash
kubectl version --short --client
aws-iam-authenticator help
```

Attach appropriate IAM role to ec2 instance

1. Turn off temporary credentials in Cloud9
2. Remove credentials → `rm -vf ${HOME}/.aws/credentials`
3. Find the instance in the AWS Console, EC2 page
4. Actions→ Attach/replace IAM role
5. Set the role

Verify the role:

```bash
INSTANCE_PROFILE_NAME=`basename $(aws ec2 describe-instances --filters Name=tag:Name,Values=aws-cloud9-${C9_PROJECT}-${C9_PID} | jq -r '.Reservations[0].Instances[0].IamInstanceProfile.Arn' | awk -F "/" "{print $2}")`
aws iam get-instance-profile --instance-profile-name $INSTANCE_PROFILE_NAME --query "InstanceProfile.Roles[0].RoleName" --output text

```

Install `eksctl` to create the cluster:

```bash
curl --silent --location "https://github.com/weaveworks/eksctl/releases/download/latest_release/eksctl_$(uname -s)_amd64.tar.gz" | tar xz -C /tmp

sudo mv -v /tmp/eksctl /usr/local/bin
```

Launch the cluster

```bash
AWS_REGION=us-east-1
eksctl create cluster --name=eksworkshop-eksctl --nodes=3 --node-ami=auto --region=${AWS_REGION}
```

Install Helm:

```jsx
cd ~/environment

curl https://raw.githubusercontent.com/kubernetes/helm/master/scripts/get > get_helm.sh

chmod +x get_helm.sh

./get_helm.sh
```

then

```jsx
cat <<EoF > ~/environment/rbac.yaml
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: tiller
  namespace: kube-system
---
apiVersion: rbac.authorization.k8s.io/v1beta1
kind: ClusterRoleBinding
metadata:
  name: tiller
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: cluster-admin
subjects:
  - kind: ServiceAccount
    name: tiller
    namespace: kube-system
EoF

kubectl apply -f ~/environment/rbac.yaml

helm init --service-account tiller
```