//! This is a rustlang library to deploy an application on target kubernetes provider
//!
//! ## About 
//! 
//! FIXME-DOCS
//! 
//! ---
//! 
//! ABSTRACT: Automatically deploy the target application on expected kubernetes provider
//! - Expected kubernetes providers: OpenShift, GKE, (Azure), AWS.
//! 
//! ## Platform compatibility
//! - [ ] Linux
//! - [ ] FreeBSD
//! - [ ] MacOS
//! - [ ] RedoxOS
//! - [ ] Windows
//! - [ ] ReactOS
//! - Others -> Support everything

// NOTICE: Pseudocoding for abstract

// FIXME: Change the name on something nice
fn kreyshifter() {
	// FIXME: This function is expects arguments specifiying what it's supposed to do

	fn sjkghasdjkgh () {
		// FIXME: Can't we just make a POST request?
		if --provider openshift {
			if release version {
				fetch https://mirror.openshift.com/pub/openshift-v4/clients/ocp/latest/openshift-install-linux.tar.gz
			} else if dev version {
				fetch https://cloud.redhat.com/openshift/install/pre-release
			} else whatever

			check if it matches checksum?

			cache it

			extract whatever was fetched to target dir

			Make it executable

			Configure `clouds.yaml` -> We need to implement master logic that parses value in these yaml files expected by kubernetes provider

			cat <<-EOF > "$targetDir/clouds.yaml"
				clouds:
					$clusterName:
						auth:
						auth_url: http://10.10.14.42:5000/v3
						project_name: $clusterName
						username: $clusterName_user
						password: passw0rd
						user_domain_name: Default
						project_domain_name: Default
					dev-env:
						region_name: RegionOne
							auth:
								username: '$clusterName_devuser'
								password: passw0rd
								project_name: '$clusterName_devuser'
								auth_url: 'https://10.10.14.22:5001/v2.0'
			EOF

			or alike

			Run it?
	}

	// kreyshifter --provider openshift -> Setup openshift environment
	switch.. arg management
}
