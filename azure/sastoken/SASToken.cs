using System;
using System.Threading.Tasks;
using Azure.Identity;
using Azure.Security.KeyVault.Secrets;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using Microsoft.WindowsAzure.Storage;
using Microsoft.WindowsAzure.Storage.Auth;
using Microsoft.WindowsAzure.Storage.Blob;
using System.IO;

namespace sastoken
{
    public static class SASToken
    {
        [FunctionName("SASToken")]
        public static async Task<IActionResult> Run(
            [HttpTrigger(AuthorizationLevel.Function, "get", "post", Route = null)] HttpRequest req,
            ILogger log)
        {
            log.LogInformation("SASToken HTTP trigger function is processing a request.");
            
            // Configuration data to get a new SAS Token.
            string storageAccountName = "account_name";
            string storageAccountKey = "account_key";
            string containerName = "container_name";
            string blobName = "blob_name";

            CloudStorageAccount storageAccount = new CloudStorageAccount(
                new StorageCredentials(storageAccountName, storageAccountKey), true);

            CloudBlobClient blobClient = storageAccount.CreateCloudBlobClient();
            CloudBlobContainer container = blobClient.GetContainerReference(containerName);
            CloudBlockBlob blob = container.GetBlockBlobReference(blobName);

            string sasToken = blob.GetSharedAccessSignature(new SharedAccessBlobPolicy()
            {
                Permissions = SharedAccessBlobPermissions.Read,
                SharedAccessExpiryTime = DateTime.UtcNow.AddHours(1),
            });

            log.LogInformation($"SASToken created: {sasToken}");

            // Configuration data to set a new o replace a secrete in Key Vault.
            string keyVaultName = "key_vault_name";
            string secretName = "secret_name";
            string secretValue = await new StreamReader(req.Body).ReadToEndAsync();

            string tenantId = "tenant_id";
            string clientId = "client_id";
            string clientSecret = "client_secret";

            var credential = new ClientSecretCredential(tenantId, clientId, clientSecret);
            var client = new SecretClient(new Uri($"https://{keyVaultName}.vault.azure.net/"), credential);

            KeyVaultSecret secret = new KeyVaultSecret(secretName, sasToken);
            Azure.Response<KeyVaultSecret> response = await client.SetSecretAsync(secret);
            log.LogInformation("Secret created or replaced.");

            return new OkResult();
        }
    }
}
